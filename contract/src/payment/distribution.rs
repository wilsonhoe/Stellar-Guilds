use crate::events::emit::emit_event;
use crate::events::topics::{
    ACT_CANCELLED, ACT_DISTRIBUTED, ACT_FAILED, ACT_RECIPIENT_ADDED, MOD_PAYMENT,
    ACT_CREATED,
};
use crate::payment::storage::{
    add_recipient_to_pool, clear_pool_recipients, get_next_pool_id, get_payment_pool,
    get_pool_recipients, recipient_exists_in_pool, store_payment_pool, update_pool_status,
};
use crate::payment::types::{
    DistributionExecutedEvent, DistributionFailedEvent, DistributionRule, DistributionStatus,
    PaymentPool, PaymentPoolCreatedEvent, PoolCancelledEvent, Recipient, RecipientAddedEvent,
};
use soroban_sdk::{contracterror, Address, Env, String, Vec};

/// Error types for payment distribution operations
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PaymentError {
    PoolNotFound = 1,
    PoolNotPending = 2,
    Unauthorized = 3,
    InvalidShare = 4,
    DuplicateRecipient = 5,
    SharesNot100Percent = 6,
    NoRecipients = 7,
    InsufficientBalance = 8,
    TransferFailed = 9,
    ArithmeticOverflow = 10,
    InvalidAmount = 11,
}

/// Minimum share amount to avoid dust issues
const MIN_SHARE_AMOUNT: i128 = 1;

/// Create a new payment pool
///
/// # Events emitted
/// - `(payment, created)` â†’ `PaymentPoolCreatedEvent`
///
/// # Arguments
/// * `env`     - The contract environment
/// * `amount`  - Total amount to distribute (must be > 0)
/// * `token`   - Token contract address (None for native XLM)
/// * `rule`    - Distribution rule type
/// * `creator` - Address creating the pool
///
/// # Returns
/// The ID of the newly created pool
pub fn create_payment_pool(
    env: &Env,
    amount: i128,
    token: Option<Address>,
    rule: DistributionRule,
    creator: Address,
) -> Result<u64, PaymentError> {
    if amount <= 0 {
        return Err(PaymentError::InvalidAmount);
    }

    let pool_id = get_next_pool_id(env);

    let pool = PaymentPool {
        id: pool_id,
        total_amount: amount,
        token: token.clone(),
        status: DistributionStatus::Pending,
        created_by: creator.clone(),
        rule: rule.clone(),
        created_at: env.ledger().timestamp(),
    };
    store_payment_pool(env, &pool);

    emit_event(
        env,
        MOD_PAYMENT,
        ACT_CREATED,
        PaymentPoolCreatedEvent {
            pool_id,
            creator,
            total_amount: amount,
            token,
            rule,
        },
    );

    Ok(pool_id)
}

/// Add a recipient to a payment pool
///
/// # Events emitted
/// - `(payment, recipient_added)` â†’ `RecipientAddedEvent`
///
/// # Arguments
/// * `env`     - The contract environment
/// * `pool_id` - ID of the pool
/// * `address` - Recipient address
/// * `share`   - Share percentage (0â€“100) or weight; meaning depends on pool rule
/// * `caller`  - Address making the request (must be pool creator)
///
/// # Errors
/// `PoolNotFound`, `Unauthorized`, `PoolNotPending`, `DuplicateRecipient`,
/// `InvalidShare`
pub fn add_recipient(
    env: &Env,
    pool_id: u64,
    address: Address,
    share: u32,
    caller: Address,
) -> Result<bool, PaymentError> {
    let pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;

    if pool.created_by != caller {
        return Err(PaymentError::Unauthorized);
    }
    if pool.status != DistributionStatus::Pending {
        return Err(PaymentError::PoolNotPending);
    }
    if recipient_exists_in_pool(env, pool_id, &address) {
        return Err(PaymentError::DuplicateRecipient);
    }

    match pool.rule {
        DistributionRule::Percentage => {
            if share == 0 || share > 100 {
                return Err(PaymentError::InvalidShare);
            }
        }
        DistributionRule::EqualSplit | DistributionRule::Weighted => {
            if share == 0 {
                return Err(PaymentError::InvalidShare);
            }
        }
    }

    let recipient = Recipient {
        address: address.clone(),
        share,
    };
    add_recipient_to_pool(env, pool_id, &recipient);

    emit_event(
        env,
        MOD_PAYMENT,
        ACT_RECIPIENT_ADDED,
        RecipientAddedEvent {
            pool_id,
            recipient: address,
            share,
        },
    );

    Ok(true)
}

/// Validate that a pool's distribution rules are met before execution.
///
/// For `Percentage` pools: all recipient shares must sum to exactly 100.
/// For `EqualSplit` / `Weighted` pools: at least one recipient must exist.
///
/// # Returns
/// `true` if validation passes; `Err` otherwise.
pub fn validate_distribution(env: &Env, pool_id: u64) -> Result<bool, PaymentError> {
    let pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;
    let recipients = get_pool_recipients(env, pool_id);

    if recipients.is_empty() {
        return Err(PaymentError::NoRecipients);
    }

    match pool.rule {
        DistributionRule::Percentage => {
            let total_percentage: u32 = recipients.iter().map(|r| r.share).sum();
            if total_percentage != 100 {
                return Err(PaymentError::SharesNot100Percent);
            }
        }
        DistributionRule::EqualSplit | DistributionRule::Weighted => {}
    }

    Ok(true)
}

/// Calculate the amount a single recipient should receive.
fn calculate_recipient_amount(
    pool: &PaymentPool,
    recipient: &Recipient,
    total_recipients: u32,
    total_weight: Option<u32>,
) -> Result<i128, PaymentError> {
    match pool.rule {
        DistributionRule::Percentage => {
            let amount = (pool.total_amount as i128)
                .checked_mul(recipient.share as i128)
                .ok_or(PaymentError::ArithmeticOverflow)?
                .checked_div(100)
                .ok_or(PaymentError::ArithmeticOverflow)?;
            Ok(amount)
        }
        DistributionRule::EqualSplit => {
            let amount = pool
                .total_amount
                .checked_div(total_recipients as i128)
                .ok_or(PaymentError::ArithmeticOverflow)?;
            Ok(amount)
        }
        DistributionRule::Weighted => {
            if let Some(total_w) = total_weight {
                let amount = (pool.total_amount as i128)
                    .checked_mul(recipient.share as i128)
                    .ok_or(PaymentError::ArithmeticOverflow)?
                    .checked_div(total_w as i128)
                    .ok_or(PaymentError::ArithmeticOverflow)?;
                Ok(amount)
            } else {
                Err(PaymentError::ArithmeticOverflow)
            }
        }
    }
}

/// Execute the distribution for a payment pool.
///
/// Transfers tokens to each recipient according to the pool's distribution rule.
/// On success emits `(payment, distributed)`; on insufficient balance emits
/// `(payment, failed)` and returns `Err(InsufficientBalance)`.
///
/// # Events emitted
/// - `(payment, distributed)` â†’ `DistributionExecutedEvent`   (on success)
/// - `(payment, failed)`      â†’ `DistributionFailedEvent`     (on failure)
///
/// # Arguments
/// * `env`     - The contract environment
/// * `pool_id` - ID of the pool to execute
/// * `caller`  - Address making the request (must be pool creator)
pub fn execute_distribution(
    env: &Env,
    pool_id: u64,
    caller: Address,
) -> Result<bool, PaymentError> {
    let mut pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;

    if pool.created_by != caller {
        return Err(PaymentError::Unauthorized);
    }
    if pool.status != DistributionStatus::Pending {
        return Err(PaymentError::PoolNotPending);
    }

    validate_distribution(env, pool_id)?;

    let recipients = get_pool_recipients(env, pool_id);
    let total_recipients = recipients.len() as u32;

    let total_weight = if pool.rule == DistributionRule::Weighted {
        Some(recipients.iter().map(|r| r.share).sum())
    } else {
        None
    };

    // Check contract balance
    let contract_balance = if let Some(token_addr) = &pool.token {
        let token_client = soroban_sdk::token::Client::new(env, token_addr);
        token_client.balance(&env.current_contract_address())
    } else {
        i128::MAX // TODO: implement native XLM balance check
    };

    if contract_balance < pool.total_amount {
        update_pool_status(env, pool_id, DistributionStatus::Failed);
        emit_event(
            env,
            MOD_PAYMENT,
            ACT_FAILED,
            DistributionFailedEvent {
                pool_id,
                reason: String::from_str(env, "Insufficient contract balance"),
            },
        );
        return Err(PaymentError::InsufficientBalance);
    }

    let mut total_distributed = 0i128;

    for recipient in recipients.iter() {
        let amount = calculate_recipient_amount(&pool, &recipient, total_recipients, total_weight)?;

        if amount < MIN_SHARE_AMOUNT {
            continue;
        }

        if let Some(token_addr) = &pool.token {
            let token_client = soroban_sdk::token::Client::new(env, token_addr);
            token_client.transfer(&env.current_contract_address(), &recipient.address, &amount);
        }
        // TODO: native XLM transfer

        total_distributed = total_distributed
            .checked_add(amount)
            .ok_or(PaymentError::ArithmeticOverflow)?;
    }

    pool.status = DistributionStatus::Executed;
    store_payment_pool(env, &pool);

    emit_event(
        env,
        MOD_PAYMENT,
        ACT_DISTRIBUTED,
        DistributionExecutedEvent {
            pool_id,
            total_recipients,
            total_distributed,
        },
    );

    Ok(true)
}

/// Get the calculated amount a specific recipient would receive.
pub fn get_recipient_amount(
    env: &Env,
    pool_id: u64,
    address: Address,
) -> Result<i128, PaymentError> {
    let pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;
    let recipients = get_pool_recipients(env, pool_id);

    let recipient = recipients
        .iter()
        .find(|r| r.address == address)
        .ok_or(PaymentError::PoolNotFound)?;

    let total_recipients = recipients.len() as u32;
    let total_weight = if pool.rule == DistributionRule::Weighted {
        Some(recipients.iter().map(|r| r.share).sum())
    } else {
        None
    };

    calculate_recipient_amount(&pool, &recipient, total_recipients, total_weight)
}

/// Cancel a pending payment pool and clear its recipients.
///
/// # Events emitted
/// - `(payment, cancelled)` â†’ `PoolCancelledEvent`
pub fn cancel_distribution(env: &Env, pool_id: u64, caller: Address) -> Result<bool, PaymentError> {
    let pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;

    if pool.created_by != caller {
        return Err(PaymentError::Unauthorized);
    }
    if pool.status != DistributionStatus::Pending {
        return Err(PaymentError::PoolNotPending);
    }

    update_pool_status(env, pool_id, DistributionStatus::Cancelled);
    clear_pool_recipients(env, pool_id);

    emit_event(
        env,
        MOD_PAYMENT,
        ACT_CANCELLED,
        PoolCancelledEvent {
            pool_id,
            cancelled_by: caller,
        },
    );

    Ok(true)
}

/// Get the current status of a payment pool.
pub fn get_pool_status(env: &Env, pool_id: u64) -> Result<DistributionStatus, PaymentError> {
    let pool = get_payment_pool(env, pool_id).ok_or(PaymentError::PoolNotFound)?;
    Ok(pool.status)
}

/// Execute distributions for multiple pools in a single call.
///
/// Returns a vector of `bool` â€” `true` for each pool that distributed
/// successfully, `false` for those that failed (individual errors are
/// captured in the `(payment, failed)` events emitted per pool).
pub fn batch_distribute(env: &Env, pool_ids: Vec<u64>, caller: Address) -> Vec<bool> {
    let mut results = Vec::new(env);
    for pool_id in pool_ids.iter() {
        let result = execute_distribution(env, pool_id, caller.clone()).is_ok();
        results.push_back(result);
    }
    results
}

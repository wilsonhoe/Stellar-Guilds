use crate::multisig::policy::ms_get_operation_policy;
use crate::multisig::storage::{
    get_account, get_operation, next_operation_id, store_account, store_operation, DataKey,
};
use crate::multisig::types::{
    AccountStatus, MultiSigOperation, OperationStatus, OperationType, TIMEOUT_24H, TIMEOUT_48H,
};
use soroban_sdk::{Address, Env, String, Vec};

pub fn ms_propose_operation(
    env: &Env,
    account_id: u64,
    op_type: OperationType,
    description: String,
    proposer: Address,
) -> Result<u64, u32> {
    proposer.require_auth();
    let mut account = get_account(env, account_id).ok_or(1u32)?;
    if !account.signers.contains(&proposer) || account.status == AccountStatus::Frozen {
        return Err(2u32);
    }
    let policy = ms_get_operation_policy(env, account_id, op_type.clone());
    let op_id = next_operation_id(env);
    let current_time = env.ledger().timestamp();
    let mut signatures = Vec::new(env);
    signatures.push_back(proposer.clone());
    let timeout = policy.timeout_seconds.clamp(TIMEOUT_24H, TIMEOUT_48H);
    let nonce = account.nonce;
    account.nonce += 1;
    store_account(env, account.id, &account);
    let operation = MultiSigOperation {
        id: op_id,
        account_id,
        op_type,
        description,
        proposer,
        signatures,
        nonce,
        created_at: current_time,
        expires_at: current_time + timeout,
        status: OperationStatus::Pending,
    };
    store_operation(env, op_id, &operation);
    Ok(op_id)
}

pub fn ms_sign_operation(env: &Env, op_id: u64, signer: Address) -> Result<u32, u32> {
    signer.require_auth();
    let mut operation = get_operation(env, op_id).ok_or(3u32)?;
    let account = get_account(env, operation.account_id).ok_or(1u32)?;
    if operation.status != OperationStatus::Pending {
        return Err(4u32);
    }
    if env.ledger().timestamp() > operation.expires_at {
        operation.status = OperationStatus::Expired;
        store_operation(env, op_id, &operation);
        return Err(5u32);
    }
    if !account.signers.contains(&signer) || operation.signatures.contains(&signer) {
        return Err(6u32);
    }
    operation.signatures.push_back(signer);
    let sig_count = operation.signatures.len();
    store_operation(env, op_id, &operation);
    Ok(sig_count)
}

pub fn ms_execute_operation(env: &Env, op_id: u64, executor: Address) -> Result<(), u32> {
    executor.require_auth();
    let mut operation = get_operation(env, op_id).ok_or(3u32)?;
    let account = get_account(env, operation.account_id).ok_or(1u32)?;
    if operation.status != OperationStatus::Pending {
        return Err(4u32);
    }
    if env.ledger().timestamp() > operation.expires_at {
        operation.status = OperationStatus::Expired;
        store_operation(env, op_id, &operation);
        return Err(5u32);
    }
    let policy = ms_get_operation_policy(env, account.id, operation.op_type.clone());
    let required_sigs = if policy.require_all_signers {
        account.signers.len()
    } else if policy.min_signatures > 0 {
        policy.min_signatures
    } else {
        account.threshold
    };
    if operation.signatures.len() < required_sigs {
        return Err(7u32);
    }
    if policy.require_owner_signature && !operation.signatures.contains(&account.owner) {
        return Err(8u32);
    }
    operation.status = OperationStatus::Executed;
    store_operation(env, op_id, &operation);
    Ok(())
}

pub fn ms_cancel_operation(env: &Env, op_id: u64, caller: Address) -> Result<(), u32> {
    caller.require_auth();
    let mut op = get_operation(env, op_id).ok_or(3u32)?;
    let account = get_account(env, op.account_id).ok_or(1u32)?;
    if op.proposer != caller && account.owner != caller {
        return Err(2u32);
    }
    if op.status != OperationStatus::Pending {
        return Err(4u32);
    }
    op.status = OperationStatus::Cancelled;
    store_operation(env, op_id, &op);
    Ok(())
}

pub fn ms_check_and_expire(env: &Env, op_id: u64) -> Result<bool, u32> {
    let mut op = get_operation(env, op_id).ok_or(3u32)?;
    if op.status == OperationStatus::Pending && env.ledger().timestamp() > op.expires_at {
        op.status = OperationStatus::Expired;
        store_operation(env, op_id, &op);
        return Ok(true);
    }
    Ok(false)
}

pub fn ms_emergency_expire_operation(env: &Env, op_id: u64, owner: Address) -> Result<(), u32> {
    owner.require_auth();
    let mut operation = get_operation(env, op_id).ok_or(3u32)?;
    let account = get_account(env, operation.account_id).ok_or(1u32)?;
    if account.owner != owner {
        return Err(2u32);
    }
    operation.status = OperationStatus::Expired;
    store_operation(env, op_id, &operation);
    Ok(())
}

pub fn ms_emergency_extend_timeout(
    env: &Env,
    op_id: u64,
    new_timeout_seconds: u64,
    owner: Address,
) -> Result<(), u32> {
    owner.require_auth();
    let mut op = get_operation(env, op_id).ok_or(3u32)?;
    let account = get_account(env, op.account_id).ok_or(1u32)?;
    if account.owner != owner {
        return Err(2u32);
    }
    let timeout = new_timeout_seconds.clamp(TIMEOUT_24H, TIMEOUT_48H);
    op.expires_at = env.ledger().timestamp() + timeout;
    store_operation(env, op_id, &op);
    Ok(())
}

pub fn ms_get_operation_status(env: &Env, op_id: u64) -> Result<MultiSigOperation, u32> {
    get_operation(env, op_id).ok_or(3u32)
}

pub fn ms_require_executed_operation(
    env: &Env,
    op_id: u64,
    expected_type: OperationType,
) -> Result<(), u32> {
    let op = get_operation(env, op_id).ok_or(3u32)?;
    if op.status != OperationStatus::Executed {
        return Err(4u32);
    }
    if op.op_type != expected_type {
        return Err(9u32);
    }
    Ok(())
}

pub fn ms_get_pending_operations(env: &Env, account_id: u64) -> Vec<MultiSigOperation> {
    let now = env.ledger().timestamp();
    let max_id: u64 = env
        .storage()
        .instance()
        .get(&DataKey::OperationCounter)
        .unwrap_or(0);
    let mut out = Vec::new(env);
    for op_id in 1..=max_id {
        if let Some(op) = get_operation(env, op_id) {
            if op.account_id == account_id
                && op.status == OperationStatus::Pending
                && now <= op.expires_at
            {
                out.push_back(op);
            }
        }
    }
    out
}

pub fn ms_sweep_expired_operations(env: &Env, account_id: u64) -> u32 {
    let now = env.ledger().timestamp();
    let max_id: u64 = env
        .storage()
        .instance()
        .get(&DataKey::OperationCounter)
        .unwrap_or(0);
    let mut expired = 0u32;
    for op_id in 1..=max_id {
        if let Some(mut op) = get_operation(env, op_id) {
            if op.account_id == account_id
                && op.status == OperationStatus::Pending
                && now > op.expires_at
            {
                op.status = OperationStatus::Expired;
                store_operation(env, op_id, &op);
                expired += 1;
            }
        }
    }
    expired
}

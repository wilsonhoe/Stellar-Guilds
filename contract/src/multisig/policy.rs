use crate::multisig::storage::{get_account, get_policy, store_policy};
use crate::multisig::types::{
    OperationPolicy, OperationType, DEFAULT_TIMEOUT, TIMEOUT_24H, TIMEOUT_48H,
};
use soroban_sdk::{Address, Env};

pub fn ms_set_operation_policy(
    env: &Env,
    account_id: u64,
    operation_type: OperationType,
    min_signatures: u32,
    require_all_signers: bool,
    timeout_seconds: u64,
    require_owner_signature: bool,
    caller: Address,
) -> Result<(), u32> {
    caller.require_auth();
    let account = get_account(env, account_id).ok_or(1u32)?;

    if account.owner != caller {
        return Err(2u32);
    }

    let min_safe = (account.signers.len() / 2) + 1;
    if !require_all_signers && (min_signatures < min_safe || min_signatures > account.signers.len())
    {
        return Err(1u32);
    }
    let timeout = if timeout_seconds == 0 {
        DEFAULT_TIMEOUT
    } else {
        timeout_seconds.clamp(TIMEOUT_24H, TIMEOUT_48H)
    };

    let policy = OperationPolicy {
        min_signatures,
        require_all_signers,
        timeout_seconds: timeout,
        require_owner_signature,
    };

    store_policy(env, account_id, operation_type, &policy);
    Ok(())
}

pub fn ms_get_operation_policy(
    env: &Env,
    account_id: u64,
    operation_type: OperationType,
) -> OperationPolicy {
    get_policy(env, account_id, operation_type).unwrap_or(OperationPolicy {
        min_signatures: 1,
        require_all_signers: false,
        timeout_seconds: DEFAULT_TIMEOUT,
        require_owner_signature: false,
    })
}

pub fn ms_reset_operation_policy(
    env: &Env,
    account_id: u64,
    operation_type: OperationType,
    caller: Address,
) -> Result<(), u32> {
    caller.require_auth();
    let account = get_account(env, account_id).ok_or(1u32)?;
    if account.owner != caller {
        return Err(2u32);
    }

    let default_policy = OperationPolicy {
        min_signatures: 1,
        require_all_signers: false,
        timeout_seconds: DEFAULT_TIMEOUT,
        require_owner_signature: false,
    };
    store_policy(env, account_id, operation_type, &default_policy);
    Ok(())
}

use crate::multisig::storage::{get_account, next_account_id, store_account};
use crate::multisig::types::{AccountStatus, MultiSigAccount};
use soroban_sdk::{Address, Env, Vec};

pub fn ms_register_account(
    env: &Env,
    owner: Address,
    mut signers: Vec<Address>,
    threshold: u32,
    _guild_id: Option<u64>,
    _timeout_seconds: u64,
) -> Result<u64, u32> {
    owner.require_auth();
    if !signers.contains(&owner) {
        signers.push_back(owner.clone());
    }
    let min_safe_threshold = (signers.len() / 2) + 1;
    if threshold < min_safe_threshold || threshold > signers.len() {
        return Err(1u32);
    }
    let account_id = next_account_id(env);
    let account = MultiSigAccount {
        id: account_id,
        owner,
        signers,
        threshold,
        status: AccountStatus::Active,
        nonce: 0,
    };
    store_account(env, account_id, &account);
    Ok(account_id)
}

pub fn ms_freeze_account(env: &Env, account_id: u64, caller: Address) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller {
        return Err(3u32);
    }
    account.status = AccountStatus::Frozen;
    store_account(env, account_id, &account);
    Ok(())
}

pub fn ms_unfreeze_account(env: &Env, account_id: u64, caller: Address) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller {
        return Err(3u32);
    }
    account.status = AccountStatus::Active;
    store_account(env, account_id, &account);
    Ok(())
}

pub fn ms_add_signer(
    env: &Env,
    account_id: u64,
    new_signer: Address,
    caller: Address,
) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller {
        return Err(3u32);
    }
    if !account.signers.contains(&new_signer) {
        account.signers.push_back(new_signer);
        store_account(env, account_id, &account);
    }
    Ok(())
}

pub fn ms_remove_signer(
    env: &Env,
    account_id: u64,
    signer: Address,
    caller: Address,
    new_threshold: u32,
) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller || account.owner == signer {
        return Err(3u32);
    }
    if let Some(idx) = account.signers.first_index_of(&signer) {
        account.signers.remove(idx);
        if account.signers.is_empty() {
            return Err(1u32);
        }
        let min_safe = (account.signers.len() / 2) + 1;
        if new_threshold < min_safe || new_threshold > account.signers.len() {
            return Err(1u32);
        }
        account.threshold = new_threshold;
        account.nonce += 1;
        store_account(env, account_id, &account);
    }
    Ok(())
}

pub fn ms_rotate_signer(
    env: &Env,
    account_id: u64,
    old_signer: Address,
    new_signer: Address,
    caller: Address,
) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller {
        return Err(3u32);
    }
    if account.signers.contains(&new_signer) {
        return Err(1u32);
    }
    if let Some(idx) = account.signers.first_index_of(&old_signer) {
        account.signers.set(idx, new_signer);
        if account.owner == old_signer {
            account.owner = account.signers.get(idx).unwrap();
        }
        account.nonce += 1;
        store_account(env, account_id, &account);
        return Ok(());
    }
    Err(4u32)
}

pub fn ms_update_threshold(
    env: &Env,
    account_id: u64,
    new_threshold: u32,
    caller: Address,
) -> Result<(), u32> {
    caller.require_auth();
    let mut account = get_account(env, account_id).ok_or(2u32)?;
    if account.owner != caller {
        return Err(3u32);
    }
    let min_safe = (account.signers.len() / 2) + 1;
    if new_threshold < min_safe || new_threshold > account.signers.len() {
        return Err(1u32);
    }
    account.threshold = new_threshold;
    account.nonce += 1;
    store_account(env, account_id, &account);
    Ok(())
}

pub fn ms_get_safe_account(env: &Env, account_id: u64) -> Result<MultiSigAccount, u32> {
    get_account(env, account_id).ok_or(2u32)
}

pub fn ms_list_accounts_by_owner(env: &Env, _owner: Address) -> Vec<MultiSigAccount> {
    let mut out = Vec::new(env);
    let max_id: u64 = env
        .storage()
        .instance()
        .get(&crate::multisig::storage::DataKey::AccountCounter)
        .unwrap_or(0);
    for id in 1..=max_id {
        if let Some(account) = get_account(env, id) {
            if account.owner == _owner {
                out.push_back(account);
            }
        }
    }
    out
}

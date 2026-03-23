use crate::multisig::types::{MultiSigAccount, MultiSigOperation, OperationPolicy, OperationType};
use soroban_sdk::{contracttype, Env};

#[contracttype]
pub enum DataKey {
    MultiSigAccount(u64),
    MultiSigOperation(u64),
    OperationPolicy(u64, OperationType),
    AccountCounter,
    OperationCounter,
}

pub fn next_account_id(env: &Env) -> u64 {
    let mut count: u64 = env
        .storage()
        .instance()
        .get(&DataKey::AccountCounter)
        .unwrap_or(0);
    count += 1;
    env.storage()
        .instance()
        .set(&DataKey::AccountCounter, &count);
    count
}

pub fn next_operation_id(env: &Env) -> u64 {
    let mut count: u64 = env
        .storage()
        .instance()
        .get(&DataKey::OperationCounter)
        .unwrap_or(0);
    count += 1;
    env.storage()
        .instance()
        .set(&DataKey::OperationCounter, &count);
    count
}

pub fn store_account(env: &Env, id: u64, account: &MultiSigAccount) {
    env.storage()
        .persistent()
        .set(&DataKey::MultiSigAccount(id), account);
}

pub fn get_account(env: &Env, id: u64) -> Option<MultiSigAccount> {
    env.storage()
        .persistent()
        .get(&DataKey::MultiSigAccount(id))
}

pub fn store_operation(env: &Env, id: u64, operation: &MultiSigOperation) {
    env.storage()
        .persistent()
        .set(&DataKey::MultiSigOperation(id), operation);
}

pub fn get_operation(env: &Env, id: u64) -> Option<MultiSigOperation> {
    env.storage()
        .persistent()
        .get(&DataKey::MultiSigOperation(id))
}

pub fn store_policy(env: &Env, account_id: u64, op_type: OperationType, policy: &OperationPolicy) {
    env.storage()
        .persistent()
        .set(&DataKey::OperationPolicy(account_id, op_type), policy);
}

pub fn get_policy(env: &Env, account_id: u64, op_type: OperationType) -> Option<OperationPolicy> {
    env.storage()
        .persistent()
        .get(&DataKey::OperationPolicy(account_id, op_type))
}

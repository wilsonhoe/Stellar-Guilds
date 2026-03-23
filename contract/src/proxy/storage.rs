use crate::proxy::types::{ProxyConfig, UpgradeTransaction};
use soroban_sdk::{symbol_short, Address, Env, Map, Symbol};

// Storage keys for proxy functionality
const PROXY_CONFIG_KEY: Symbol = symbol_short!("prx_cfg");
const UPGRADE_HISTORY_KEY: Symbol = symbol_short!("upg_hist");
const IMPLEMENTATION_SLOT: Symbol = symbol_short!("impl_slot");
const ADMIN_SLOT: Symbol = symbol_short!("adm_slot");

/// Initialize proxy storage
pub fn initialize(env: &Env, initial_implementation: Address, admin: Address) {
    let config = ProxyConfig {
        implementation: initial_implementation,
        admin,
        version: 1, // Start with version 1
        last_updated: env.ledger().timestamp(),
    };
    
    env.storage().persistent().set(&PROXY_CONFIG_KEY, &config);
    
    // Also store implementation in a dedicated slot for easy access
    env.storage().persistent().set(&IMPLEMENTATION_SLOT, &config.implementation);
    
    // Initialize upgrade history
    let upgrade_history: Map<u64, UpgradeTransaction> = Map::new(env);
    env.storage().persistent().set(&UPGRADE_HISTORY_KEY, &upgrade_history);
}

/// Get the current proxy configuration
pub fn get_proxy_config(env: &Env) -> ProxyConfig {
    env.storage()
        .persistent()
        .get(&PROXY_CONFIG_KEY)
        .expect("Proxy config not initialized")
}

/// Get the current implementation address
pub fn get_implementation(env: &Env) -> Address {
    env.storage()
        .persistent()
        .get(&IMPLEMENTATION_SLOT)
        .expect("Implementation address not set")
}

/// Set a new implementation address
pub fn set_implementation(env: &Env, implementation: &Address) {
    env.storage().persistent().set(&IMPLEMENTATION_SLOT, implementation);
    
    // Also update the config
    let mut config = get_proxy_config(env);
    config.implementation = implementation.clone();
    config.version += 1; // Increment version
    config.last_updated = env.ledger().timestamp();
    
    env.storage().persistent().set(&PROXY_CONFIG_KEY, &config);
}

/// Get the admin address
pub fn get_admin(env: &Env) -> Address {
    let config = get_proxy_config(env);
    config.admin
}

/// Set a new admin address
pub fn set_admin(env: &Env, admin: &Address) {
    let mut config = get_proxy_config(env);
    config.admin = admin.clone();
    config.last_updated = env.ledger().timestamp();
    
    env.storage().persistent().set(&PROXY_CONFIG_KEY, &config);
}

/// Record an upgrade transaction
pub fn record_upgrade_transaction(env: &Env, transaction: &UpgradeTransaction) {
    let mut upgrade_history: Map<u64, UpgradeTransaction> = env
        .storage()
        .persistent()
        .get(&UPGRADE_HISTORY_KEY)
        .unwrap_or_else(|| Map::new(env));

    upgrade_history.set(transaction.id, transaction.clone());
    env.storage()
        .persistent()
        .set(&UPGRADE_HISTORY_KEY, &upgrade_history);
}

/// Get an upgrade transaction by ID
pub fn get_upgrade_transaction(env: &Env, id: u64) -> Option<UpgradeTransaction> {
    let upgrade_history: Map<u64, UpgradeTransaction> = env
        .storage()
        .persistent()
        .get(&UPGRADE_HISTORY_KEY)
        .unwrap_or_else(|| Map::new(env));

    upgrade_history.get(id)
}

/// Check if an address is the admin
pub fn is_admin(env: &Env, address: &Address) -> bool {
    let config = get_proxy_config(env);
    &config.admin == address
}

/// Check if an address is the current implementation
pub fn is_current_implementation(env: &Env, address: &Address) -> bool {
    let current_impl = get_implementation(env);
    &current_impl == address
}

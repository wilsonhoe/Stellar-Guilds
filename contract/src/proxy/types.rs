use soroban_sdk::{contracttype, Address};

/// Proxy contract configuration
#[contracttype]
#[derive(Clone, Debug)]
pub struct ProxyConfig {
    /// Address of the current implementation contract
    pub implementation: Address,
    /// Address of the admin/governance contract
    pub admin: Address,
    /// Current version of the implementation
    pub version: u32,
    /// Timestamp when the proxy was last updated
    pub last_updated: u64,
}

/// Represents an upgrade transaction
#[contracttype]
#[derive(Clone, Debug)]
pub struct UpgradeTransaction {
    /// ID of the upgrade transaction
    pub id: u64,
    /// Address of the new implementation
    pub new_implementation: Address,
    /// Address of the caller who initiated the upgrade
    pub initiator: Address,
    /// Timestamp of the upgrade
    pub timestamp: u64,
    /// Whether the upgrade was successful
    pub success: bool,
    /// Reason for failure if upgrade failed
    pub failure_reason: Option<soroban_sdk::String>,
}

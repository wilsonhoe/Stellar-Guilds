use soroban_sdk::{contracttype, Address};

/// Represents the current version of the contract
#[contracttype]
#[derive(Clone, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version { major, minor, patch }
    }

    /// Compare two versions for compatibility
    /// Returns true if `self` is compatible with `other`
    pub fn is_compatible_with(&self, other: &Version) -> bool {
        // Major version must match for compatibility
        self.major == other.major
        // Minor version of self should be >= other for backward compatibility
        && self.minor >= other.minor
    }
}

/// Status of an upgrade proposal
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeStatus {
    Pending = 0,
    Approved = 1,
    Executed = 2,
    Rejected = 3,
    Cancelled = 4,
}

/// Information about a proposed upgrade
#[contracttype]
#[derive(Clone, Debug)]
pub struct UpgradeProposal {
    pub id: u64,
    pub proposer: Address,
    pub new_contract_address: Address,
    pub version: Version,
    pub description: soroban_sdk::String,
    pub timestamp: u64,
    pub status: UpgradeStatus,
    pub votes_for: u32,
    pub votes_against: u32,
    pub total_voters: u32,
}

/// Represents a migration plan between contract versions
#[contracttype]
#[derive(Clone, Debug)]
pub struct MigrationPlan {
    pub from_version: Version,
    pub to_version: Version,
    pub migration_function_selector: soroban_sdk::Symbol,
    pub estimated_gas: u64,
}

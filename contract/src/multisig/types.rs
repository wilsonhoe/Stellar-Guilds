use soroban_sdk::{contracttype, Address, String, Vec};

pub const TIMEOUT_24H: u64 = 86_400;
pub const TIMEOUT_48H: u64 = 172_800;
pub const DEFAULT_TIMEOUT: u64 = TIMEOUT_48H;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Frozen,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationStatus {
    Pending,
    Executed,
    Expired,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationType {
    TreasuryWithdrawal,
    GovernanceUpdate,
    GuildConfigChange,
    EmergencyAction,
}

#[contracttype]
#[derive(Clone)]
pub struct MultiSigAccount {
    pub id: u64,
    pub owner: Address,
    pub signers: Vec<Address>,
    pub threshold: u32,
    pub status: AccountStatus,
    pub nonce: u64, // Replay protection
}

#[contracttype]
#[derive(Clone)]
pub struct MultiSigOperation {
    pub id: u64,
    pub account_id: u64,
    pub op_type: OperationType,
    pub description: String,
    pub proposer: Address,
    pub signatures: Vec<Address>,
    pub nonce: u64,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: OperationStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct OperationPolicy {
    pub min_signatures: u32,
    pub require_all_signers: bool,
    pub timeout_seconds: u64,
    pub require_owner_signature: bool,
}

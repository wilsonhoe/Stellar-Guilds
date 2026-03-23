use soroban_sdk::{contracttype, Address, String};

/// Distribution rule types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DistributionRule {
    /// Percentage-based distribution (shares must sum to 100%)
    Percentage,
    /// Equal split among all recipients
    EqualSplit,
    /// Weighted distribution based on contribution weights
    Weighted,
}

/// Status of a payment pool distribution
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DistributionStatus {
    /// Pool is pending, can still add recipients or modify
    Pending,
    /// Distribution has been executed successfully
    Executed,
    /// Distribution failed (e.g., insufficient funds, transfer errors)
    Failed,
    /// Pool was cancelled by creator
    Cancelled,
}

/// A payment pool containing funds to be distributed
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentPool {
    /// Unique pool identifier
    pub id: u64,
    /// Total amount to distribute
    pub total_amount: i128,
    /// Token contract address (use native XLM if None)
    pub token: Option<Address>,
    /// Current status of the pool
    pub status: DistributionStatus,
    /// Address that created the pool
    pub created_by: Address,
    /// Distribution rule type
    pub rule: DistributionRule,
    /// Timestamp when pool was created
    pub created_at: u64,
}

/// A recipient in a payment distribution
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recipient {
    /// Recipient address
    pub address: Address,
    /// Share percentage (0-100) for Percentage rule, or weight for Weighted rule
    pub share: u32,
}

/// Event emitted when a payment pool is created
#[contracttype]
pub struct PaymentPoolCreatedEvent {
    pub pool_id: u64,
    pub creator: Address,
    pub total_amount: i128,
    pub token: Option<Address>,
    pub rule: DistributionRule,
}

/// Event emitted when a recipient is added to a pool
#[contracttype]
pub struct RecipientAddedEvent {
    pub pool_id: u64,
    pub recipient: Address,
    pub share: u32,
}

/// Event emitted when distribution is executed
#[contracttype]
pub struct DistributionExecutedEvent {
    pub pool_id: u64,
    pub total_recipients: u32,
    pub total_distributed: i128,
}

/// Event emitted when distribution fails
#[contracttype]
pub struct DistributionFailedEvent {
    pub pool_id: u64,
    pub reason: String,
}

/// Event emitted when a pool is cancelled
#[contracttype]
pub struct PoolCancelledEvent {
    pub pool_id: u64,
    pub cancelled_by: Address,
}

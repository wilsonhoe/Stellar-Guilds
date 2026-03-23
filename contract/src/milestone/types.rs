use soroban_sdk::{contracttype, Address, String};

/// Overall status of a project
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectStatus {
    Active,
    Completed,
    Cancelled,
}

/// Status of a single milestone in a project
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Submitted,
    Approved,
    Rejected,
    Expired,
}

/// Project configuration and aggregate accounting
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub id: u64,
    pub guild_id: u64,
    pub contributor: Address,
    pub treasury_id: u64,
    pub token: Option<Address>,
    pub total_amount: i128,
    pub allocated_amount: i128,
    pub released_amount: i128,
    pub is_sequential: bool,
    pub created_at: u64,
    pub status: ProjectStatus,
}

/// Milestone metadata and state
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub id: u64,
    pub project_id: u64,
    pub order: u32,
    pub title: String,
    pub description: String,
    pub payment_amount: i128,
    pub deadline: u64,
    pub status: MilestoneStatus,
    pub proof_url: String,
    pub created_at: u64,
    pub submitted_at: Option<u64>,
    pub last_updated_at: u64,
    pub version: u32,
    pub is_payment_released: bool,
}

/// Input used when creating a project with multiple milestones
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneInput {
    pub title: String,
    pub description: String,
    pub payment_amount: i128,
    pub deadline: u64,
}

// Events

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectCreatedEvent {
    pub project_id: u64,
    pub guild_id: u64,
    pub contributor: Address,
    pub treasury_id: u64,
    pub token: Option<Address>,
    pub total_amount: i128,
    pub is_sequential: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectStatusChangedEvent {
    pub project_id: u64,
    pub old_status: ProjectStatus,
    pub new_status: ProjectStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneAddedEvent {
    pub project_id: u64,
    pub milestone_id: u64,
    pub title: String,
    pub payment_amount: i128,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneStatusChangedEvent {
    pub project_id: u64,
    pub milestone_id: u64,
    pub old_status: MilestoneStatus,
    pub new_status: MilestoneStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneSubmittedEvent {
    pub project_id: u64,
    pub milestone_id: u64,
    pub proof_url: String,
    pub version: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneRejectedEvent {
    pub project_id: u64,
    pub milestone_id: u64,
    pub reason: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestonePaymentReleasedEvent {
    pub project_id: u64,
    pub milestone_id: u64,
    pub treasury_id: u64,
    pub amount: i128,
    pub token: Option<Address>,
    pub recipient: Address,
}

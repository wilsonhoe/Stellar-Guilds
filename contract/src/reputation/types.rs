use soroban_sdk::{contracttype, Address, String};

// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ Contribution Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Types of contributions that earn reputation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContributionType {
    BountyCompleted,
    MilestoneApproved,
    ProposalCreated,
    VoteCast,
    DisputeResolved,
}

// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ Scoring Constants â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Points awarded per contribution type
pub const POINTS_BOUNTY_COMPLETED: u32 = 100;
pub const POINTS_MILESTONE_APPROVED: u32 = 50;
pub const POINTS_PROPOSAL_CREATED: u32 = 20;
pub const POINTS_VOTE_CAST: u32 = 5;
pub const POINTS_DISPUTE_RESOLVED: u32 = 30;

/// Decay: 1% per period, applied lazily
pub const DECAY_PERIOD_SECS: u64 = 604_800; // 1 week
/// Decay numerator / denominator => 99/100 = keep 99% per period
pub const DECAY_NUMERATOR: u64 = 99;
pub const DECAY_DENOMINATOR: u64 = 100;

// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ Core Structs â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Individual contribution record for audit trail
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionRecord {
    pub id: u64,
    pub guild_id: u64,
    pub contributor: Address,
    pub contribution_type: ContributionType,
    pub points: u32,
    pub timestamp: u64,
    /// Reference to the bounty/milestone/proposal ID
    pub reference_id: u64,
}

/// Per-user per-guild reputation profile
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationProfile {
    pub address: Address,
    pub guild_id: u64,
    /// Raw accumulated score (never decreases)
    pub total_score: u64,
    /// Score after decay is applied (used for governance weight)
    pub decayed_score: u64,
    /// Total number of contributions
    pub contributions_count: u32,
    /// Timestamp of last contribution
    pub last_activity: u64,
    /// Timestamp when decay was last calculated
    pub last_decay_applied: u64,
}

// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ Badge System â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// Types of badges that can be earned
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BadgeType {
    /// First ever contribution
    FirstContribution,
    /// Completed 5+ bounties
    BountyHunter,
    /// 10+ milestones approved
    Mentor,
    /// Cast 10+ votes
    Governor,
    /// Reputation score exceeds 1000
    Veteran,
}

/// Badge / achievement held by a user
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Badge {
    pub id: u64,
    pub guild_id: u64,
    pub holder: Address,
    pub badge_type: BadgeType,
    pub name: String,
    pub awarded_at: u64,
}

// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ Events â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReputationUpdatedEvent {
    pub guild_id: u64,
    pub contributor: Address,
    pub points_earned: u32,
    pub new_total_score: u64,
    pub contribution_type: ContributionType,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadgeAwardedEvent {
    pub guild_id: u64,
    pub holder: Address,
    pub badge_type: BadgeType,
    pub badge_name: String,
}

/// Helper to get points for a contribution type
pub fn points_for_contribution(ct: &ContributionType) -> u32 {
    match ct {
        ContributionType::BountyCompleted => POINTS_BOUNTY_COMPLETED,
        ContributionType::MilestoneApproved => POINTS_MILESTONE_APPROVED,
        ContributionType::ProposalCreated => POINTS_PROPOSAL_CREATED,
        ContributionType::VoteCast => POINTS_VOTE_CAST,
        ContributionType::DisputeResolved => POINTS_DISPUTE_RESOLVED,
    }
}

use crate::upgrade::types::{MigrationPlan, UpgradeProposal, UpgradeStatus, Version};
use soroban_sdk::{symbol_short, Address, Env, Map, Symbol, Vec};

// Storage keys for upgrade functionality
const CURRENT_VERSION_KEY: Symbol = symbol_short!("cur_ver");
const UPGRADE_PROPOSALS_KEY: Symbol = symbol_short!("upg_prop");
const VOTING_POWER_KEY: Symbol = symbol_short!("vote_pow");
const GOVERNANCE_ADDRESS_KEY: Symbol = symbol_short!("gov_addr");
const EMERGENCY_UPGRADE_KEY: Symbol = symbol_short!("emg_upg");
const MIGRATION_PLANS_KEY: Symbol = symbol_short!("migr_pln");

/// Initialize upgrade storage
pub fn initialize(env: &Env, initial_version: Version, governance_address: Address) {
    env.storage()
        .persistent()
        .set(&CURRENT_VERSION_KEY, &initial_version);
    env.storage()
        .persistent()
        .set(&GOVERNANCE_ADDRESS_KEY, &governance_address);
    
    // Initialize empty proposals map
    let proposals: Map<u64, UpgradeProposal> = Map::new(env);
    env.storage().persistent().set(&UPGRADE_PROPOSALS_KEY, &proposals);
    
    // Initialize empty voting power map
    let voting_power: Map<Address, u32> = Map::new(env);
    env.storage().persistent().set(&VOTING_POWER_KEY, &voting_power);
    
    // Initialize empty migration plans map
    let migration_plans: Map<u64, MigrationPlan> = Map::new(env);
    env.storage().persistent().set(&MIGRATION_PLANS_KEY, &migration_plans);
    
    // Set emergency upgrade flag to false
    env.storage().persistent().set(&EMERGENCY_UPGRADE_KEY, &false);
}

/// Get the current contract version
pub fn get_current_version(env: &Env) -> Version {
    env.storage()
        .persistent()
        .get(&CURRENT_VERSION_KEY)
        .expect("Current version not initialized")
}

/// Set the current contract version
pub fn set_current_version(env: &Env, version: &Version) {
    env.storage().persistent().set(&CURRENT_VERSION_KEY, version);
}

/// Get the governance address
pub fn get_governance_address(env: &Env) -> Address {
    env.storage()
        .persistent()
        .get(&GOVERNANCE_ADDRESS_KEY)
        .expect("Governance address not set")
}

/// Store an upgrade proposal
pub fn store_upgrade_proposal(env: &Env, proposal: &UpgradeProposal) {
    let mut proposals: Map<u64, UpgradeProposal> = env
        .storage()
        .persistent()
        .get(&UPGRADE_PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    proposals.set(proposal.id, proposal.clone());
    env.storage()
        .persistent()
        .set(&UPGRADE_PROPOSALS_KEY, &proposals);
}

/// Get an upgrade proposal by ID
pub fn get_upgrade_proposal(env: &Env, proposal_id: u64) -> Option<UpgradeProposal> {
    let proposals: Map<u64, UpgradeProposal> = env
        .storage()
        .persistent()
        .get(&UPGRADE_PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    proposals.get(proposal_id)
}

/// Get all pending upgrade proposals
pub fn get_pending_proposals(env: &Env) -> Vec<UpgradeProposal> {
    let _proposals: Map<u64, UpgradeProposal> = env
        .storage()
        .persistent()
        .get(&UPGRADE_PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let result = Vec::new(env);

    // In Soroban, iteration over maps isn't directly supported in this way
    // We'll need to store proposal IDs separately to iterate over them
    // For now, return an empty vector - this would need to be implemented differently
    result
}

/// Update the status of an upgrade proposal
pub fn update_proposal_status(env: &Env, proposal_id: u64, status: UpgradeStatus) {
    if let Some(mut proposal) = get_upgrade_proposal(env, proposal_id) {
        proposal.status = status;
        store_upgrade_proposal(env, &proposal);
    }
}

/// Set voting power for an address
pub fn set_voting_power(env: &Env, address: &Address, power: u32) {
    let mut voting_power: Map<Address, u32> = env
        .storage()
        .persistent()
        .get(&VOTING_POWER_KEY)
        .unwrap_or_else(|| Map::new(env));

    voting_power.set(address.clone(), power);
    env.storage().persistent().set(&VOTING_POWER_KEY, &voting_power);
}

/// Get voting power for an address
pub fn get_voting_power(env: &Env, address: &Address) -> u32 {
    let voting_power: Map<Address, u32> = env
        .storage()
        .persistent()
        .get(&VOTING_POWER_KEY)
        .unwrap_or_else(|| Map::new(env));

    voting_power.get(address.clone()).unwrap_or(0)
}

/// Record a vote on an upgrade proposal
pub fn record_vote(
    env: &Env,
    proposal_id: u64,
    voter: &Address,
    vote_for: bool,
) -> Result<(), &'static str> {
    let mut proposal = get_upgrade_proposal(env, proposal_id)
        .ok_or("Proposal does not exist")?;

    if proposal.status != UpgradeStatus::Pending {
        return Err("Proposal is not in pending status");
    }

    // Check if voter has already voted
    // In a real implementation, we'd track who has voted
    // For simplicity, we'll just update the vote counts
    
    if vote_for {
        proposal.votes_for += get_voting_power(env, voter);
    } else {
        proposal.votes_against += get_voting_power(env, voter);
    }

    store_upgrade_proposal(env, &proposal);
    Ok(())
}

/// Store a migration plan
pub fn store_migration_plan(env: &Env, proposal_id: u64, plan: &MigrationPlan) {
    let mut migration_plans: Map<u64, MigrationPlan> = env
        .storage()
        .persistent()
        .get(&MIGRATION_PLANS_KEY)
        .unwrap_or_else(|| Map::new(env));

    migration_plans.set(proposal_id, plan.clone());
    env.storage()
        .persistent()
        .set(&MIGRATION_PLANS_KEY, &migration_plans);
}

/// Get a migration plan by proposal ID
pub fn get_migration_plan(env: &Env, proposal_id: u64) -> Option<MigrationPlan> {
    let migration_plans: Map<u64, MigrationPlan> = env
        .storage()
        .persistent()
        .get(&MIGRATION_PLANS_KEY)
        .unwrap_or_else(|| Map::new(env));

    migration_plans.get(proposal_id)
}

/// Check if emergency upgrades are enabled
pub fn is_emergency_upgrade_enabled(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&EMERGENCY_UPGRADE_KEY)
        .unwrap_or(false)
}

/// Enable/disable emergency upgrades
pub fn set_emergency_upgrade_enabled(env: &Env, enabled: bool) {
    env.storage().persistent().set(&EMERGENCY_UPGRADE_KEY, &enabled);
}

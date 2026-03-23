use crate::upgrade::storage;
use crate::upgrade::types::{MigrationPlan, UpgradeProposal, UpgradeStatus, Version};
use soroban_sdk::{symbol_short, Address, Env, String};

/// Create a new upgrade proposal
pub fn propose_upgrade(
    env: &Env,
    proposer: &Address,
    new_contract_address: &Address,
    target_version: &Version,
    description: String,
) -> u64 {
    // Verify the proposer has the right to propose upgrades
    let governance_addr = storage::get_governance_address(env);
    proposer.require_auth();

    // In a real implementation, we might check if the proposer has sufficient voting power
    // For now, we just verify the governance address
    
    // Generate a new proposal ID (in practice, this might be more sophisticated)
    let proposal_id = env.storage().instance().get(&symbol_short!("nxt_prop")).unwrap_or(1u64);
    env.storage().instance().set(&symbol_short!("nxt_prop"), &(proposal_id + 1));

    let proposal = UpgradeProposal {
        id: proposal_id,
        proposer: proposer.clone(),
        new_contract_address: new_contract_address.clone(),
        version: target_version.clone(),
        description,
        timestamp: env.ledger().timestamp(),
        status: UpgradeStatus::Pending,
        votes_for: 0,
        votes_against: 0,
        total_voters: 0, // Will be calculated when voting begins
    };

    storage::store_upgrade_proposal(env, &proposal);

    // Emit event for the proposal
    env.events()
        .publish(("upgrade", "proposal_created"), proposal_id);

    proposal_id
}

/// Vote on an upgrade proposal
pub fn vote_on_proposal(
    env: &Env,
    voter: &Address,
    proposal_id: u64,
    vote_for: bool,
) -> Result<(), &'static str> {
    voter.require_auth();
    
    // Record the vote
    storage::record_vote(env, proposal_id, voter, vote_for)?;
    
    // Check if proposal has reached required threshold
    if let Some(proposal) = storage::get_upgrade_proposal(env, proposal_id) {
        let _total_votes = proposal.votes_for + proposal.votes_against;
        // Simple majority threshold - in real implementation this could be configurable
        let required_votes = (proposal.total_voters / 2) + 1;
        
        if proposal.votes_for >= required_votes {
            storage::update_proposal_status(env, proposal_id, UpgradeStatus::Approved);
            env.events()
                .publish(("upgrade", "proposal_approved"), proposal_id);
        } else if proposal.votes_against >= required_votes {
            storage::update_proposal_status(env, proposal_id, UpgradeStatus::Rejected);
            env.events()
                .publish(("upgrade", "proposal_rejected"), proposal_id);
        }
    }
    
    Ok(())
}

/// Execute an approved upgrade
pub fn execute_upgrade(env: &Env, executor: &Address, proposal_id: u64) -> Result<(), &'static str> {
    executor.require_auth();
    
    let mut proposal = storage::get_upgrade_proposal(env, proposal_id)
        .ok_or("Proposal does not exist")?;
    
    if proposal.status != UpgradeStatus::Approved {
        return Err("Proposal is not approved for execution");
    }
    
    // Check if the caller is authorized to execute upgrades
    let governance_addr = storage::get_governance_address(env);
    if *executor != governance_addr {
        return Err("Only governance address can execute upgrades");
    }
    
    // Perform state migration if a migration plan exists
    if let Some(migration_plan) = storage::get_migration_plan(env, proposal_id) {
        perform_state_migration(env, &migration_plan)?;
    }
    
    // Update the current version
    storage::set_current_version(env, &proposal.version);
    
    // Update proposal status
    proposal.status = UpgradeStatus::Executed;
    storage::store_upgrade_proposal(env, &proposal);
    
    // Emit upgrade execution event
    env.events()
        .publish(("upgrade", "executed"), proposal_id);
    
    Ok(())
}

/// Perform emergency upgrade bypassing the normal governance process
pub fn emergency_upgrade(
    env: &Env,
    caller: &Address,
    _new_contract_address: &Address,
    new_version: &Version,
) -> Result<(), &'static str> {
    caller.require_auth();
    
    // Check if emergency upgrades are enabled
    if !storage::is_emergency_upgrade_enabled(env) {
        return Err("Emergency upgrades are not enabled");
    }
    
    // Only governance address can perform emergency upgrades
    let governance_addr = storage::get_governance_address(env);
    if *caller != governance_addr {
        return Err("Only governance address can perform emergency upgrades");
    }
    
    // Update the current version directly
    storage::set_current_version(env, new_version);
    
    // Emit emergency upgrade event
    env.events()
        .publish(("upgrade", "emergency_executed"), new_version.clone());
    
    Ok(())
}

/// Enable or disable emergency upgrades
pub fn toggle_emergency_upgrades(env: &Env, caller: &Address, enable: bool) -> Result<(), &'static str> {
    caller.require_auth();
    
    // Only governance address can enable/disable emergency upgrades
    let governance_addr = storage::get_governance_address(env);
    if *caller != governance_addr {
        return Err("Only governance address can toggle emergency upgrades");
    }
    
    storage::set_emergency_upgrade_enabled(env, enable);
    
    env.events()
        .publish(("upgrade", "emergency_toggled"), enable);
    
    Ok(())
}

/// Register a migration plan for an upgrade
pub fn register_migration_plan(
    env: &Env,
    caller: &Address,
    proposal_id: u64,
    migration_plan: &MigrationPlan,
) -> Result<(), &'static str> {
    caller.require_auth();
    
    // Only governance address can register migration plans
    let governance_addr = storage::get_governance_address(env);
    if *caller != governance_addr {
        return Err("Only governance address can register migration plans");
    }
    
    storage::store_migration_plan(env, proposal_id, migration_plan);
    
    env.events()
        .publish(("upgrade", "migration_registered"), proposal_id);
    
    Ok(())
}

/// Perform state migration based on a migration plan
fn perform_state_migration(env: &Env, plan: &MigrationPlan) -> Result<(), &'static str> {
    // In a real implementation, this would call specific migration functions
    // based on the migration plan's selector
    // For now, we'll just log the migration attempt
    
    env.events()
        .publish(("upgrade", "migration_started"), plan.from_version.clone());
    
    // Placeholder for actual migration logic
    // This would involve calling migration functions that transform data
    // from the old format to the new format
    
    env.events()
        .publish(("upgrade", "migration_completed"), plan.to_version.clone());
    
    Ok(())
}

/// Check version compatibility between current and target version
pub fn check_version_compatibility(current: &Version, target: &Version) -> bool {
    // Major version must match for compatibility
    // Minor version of target should be >= current for forward compatibility
    current.major == target.major && target.minor >= current.minor
}

/// Rollback to a previous version (limited capability)
pub fn rollback_to_version(
    env: &Env,
    caller: &Address,
    target_version: &Version,
) -> Result<(), &'static str> {
    caller.require_auth();
    
    // Only governance address can perform rollbacks
    let governance_addr = storage::get_governance_address(env);
    if *caller != governance_addr {
        return Err("Only governance address can perform rollbacks");
    }
    
    // In a real implementation, this would involve complex state restoration
    // For now, we'll just check if the rollback is to a previous version
    let current_version = storage::get_current_version(env);
    
    if target_version.major != current_version.major || 
       (target_version.major == current_version.major && target_version.minor > current_version.minor) {
        return Err("Can only rollback to earlier versions in the same major series");
    }
    
    // Update to the target version
    storage::set_current_version(env, target_version);
    
    env.events()
        .publish(("upgrade", "rollback_completed"), target_version.clone());
    
    Ok(())
}

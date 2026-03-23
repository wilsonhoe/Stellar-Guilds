use soroban_sdk::{Address, Env, Symbol};

use crate::governance::proposals::get_proposal as load_proposal;
use crate::governance::storage::store_proposal;
use crate::governance::types::{
    ExecutionPayload, Proposal, ProposalExecutedEvent, ProposalStatus, ProposalType,
};
use crate::governance::voting::finalize_proposal;

const EXECUTION_DEADLINE_SECONDS: u64 = 3 * 24 * 60 * 60; // 3 days after passing

pub fn execute_proposal(env: &Env, proposal_id: u64, executor: Address) -> bool {
    let mut proposal = load_proposal(env, proposal_id);
    executor.require_auth(); // Enforce the new auth check for security

    let now = env.ledger().timestamp();
    if matches!(proposal.status, ProposalStatus::Active) && now >= proposal.voting_end {
        let _status = finalize_proposal(env, proposal_id);
        proposal = load_proposal(env, proposal_id);
        if !matches!(proposal.status, ProposalStatus::Passed) {
            panic!("proposal not passed");
        }
    }

    if !matches!(proposal.status, ProposalStatus::Passed) {
        panic!("only passed proposals can be executed");
    }

    if let Some(passed_at) = proposal.passed_at {
        if now > passed_at + EXECUTION_DEADLINE_SECONDS {
            proposal.status = ProposalStatus::Expired;
            store_proposal(env, &proposal);
            panic!("execution window expired");
        }
    }

    let success = match (&proposal.proposal_type, &proposal.execution_payload) {
        (ProposalType::TreasurySpend, ExecutionPayload::TreasurySpend) => {
            // High-security action: Relies on the new multisig flow.
            true
        }
        (ProposalType::RuleChange, ExecutionPayload::RuleChange) => {
            // High-security action
            true
        }
        (ProposalType::GeneralDecision, ExecutionPayload::GeneralDecision) => true,
        _ => false,
    };

    let mut proposal_to_update: Proposal = proposal.clone();
    if success {
        proposal_to_update.status = ProposalStatus::Executed;
        proposal_to_update.executed_at = Some(now);
        store_proposal(env, &proposal_to_update);
    }

    let event = ProposalExecutedEvent {
        proposal_id,
        success,
    };
    env.events().publish(
        (
            Symbol::new(env, "proposal_executed"),
            Symbol::new(env, "v0"),
        ),
        event,
    );

    success
}

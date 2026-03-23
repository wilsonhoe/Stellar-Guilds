use soroban_sdk::{symbol_short, Address, Env, Map, Symbol, Vec};

use crate::governance::types::{GovernanceConfig, Proposal, Vote};

const PROPOSALS_KEY: Symbol = symbol_short!("g_props");
const PROPOSAL_COUNTER_KEY: Symbol = symbol_short!("g_pcnt");
const GUILD_PROPOSALS_KEY: Symbol = symbol_short!("g_pidx");

const VOTES_KEY: Symbol = symbol_short!("g_votes");

const DELEGATIONS_KEY: Symbol = symbol_short!("g_deleg");

const GOV_CONFIG_KEY: Symbol = symbol_short!("g_conf");

pub fn get_next_proposal_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .persistent()
        .get(&PROPOSAL_COUNTER_KEY)
        .unwrap_or(0u64);
    let next = current + 1;
    env.storage().persistent().set(&PROPOSAL_COUNTER_KEY, &next);
    next
}

pub fn store_proposal(env: &Env, proposal: &Proposal) {
    let mut proposals: Map<u64, Proposal> = env
        .storage()
        .persistent()
        .get(&PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    proposals.set(proposal.id, proposal.clone());
    env.storage().persistent().set(&PROPOSALS_KEY, &proposals);

    // index by guild
    let mut index: Map<u64, Vec<u64>> = env
        .storage()
        .persistent()
        .get(&GUILD_PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let mut list = index
        .get(proposal.guild_id)
        .unwrap_or_else(|| Vec::new(env));
    if !list.iter().any(|id| id == proposal.id) {
        list.push_back(proposal.id);
        index.set(proposal.guild_id, list);
        env.storage().persistent().set(&GUILD_PROPOSALS_KEY, &index);
    }
}

pub fn get_proposal(env: &Env, proposal_id: u64) -> Option<Proposal> {
    let proposals: Map<u64, Proposal> = env
        .storage()
        .persistent()
        .get(&PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    proposals.get(proposal_id)
}

pub fn get_guild_proposals(env: &Env, guild_id: u64) -> Vec<Proposal> {
    let index: Map<u64, Vec<u64>> = env
        .storage()
        .persistent()
        .get(&GUILD_PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let ids = index.get(guild_id).unwrap_or_else(|| Vec::new(env));

    let proposals: Map<u64, Proposal> = env
        .storage()
        .persistent()
        .get(&PROPOSALS_KEY)
        .unwrap_or_else(|| Map::new(env));

    let mut result = Vec::new(env);
    for id in ids.iter() {
        if let Some(p) = proposals.get(id) {
            result.push_back(p);
        }
    }
    result
}

pub fn store_vote(env: &Env, vote: &Vote) {
    let mut votes_map: Map<u64, Map<Address, Vote>> = env
        .storage()
        .persistent()
        .get(&VOTES_KEY)
        .unwrap_or_else(|| Map::new(env));

    let mut proposal_votes = votes_map
        .get(vote.proposal_id)
        .unwrap_or_else(|| Map::new(env));

    proposal_votes.set(vote.voter.clone(), vote.clone());
    votes_map.set(vote.proposal_id, proposal_votes);

    env.storage().persistent().set(&VOTES_KEY, &votes_map);
}

#[allow(dead_code)]
pub fn get_vote(env: &Env, proposal_id: u64, voter: &Address) -> Option<Vote> {
    let votes_map: Map<u64, Map<Address, Vote>> = env
        .storage()
        .persistent()
        .get(&VOTES_KEY)
        .unwrap_or_else(|| Map::new(env));

    let proposal_votes = votes_map.get(proposal_id)?;
    proposal_votes.get(voter.clone())
}

pub fn get_all_votes(env: &Env, proposal_id: u64) -> Map<Address, Vote> {
    let votes_map: Map<u64, Map<Address, Vote>> = env
        .storage()
        .persistent()
        .get(&VOTES_KEY)
        .unwrap_or_else(|| Map::new(env));

    votes_map.get(proposal_id).unwrap_or_else(|| Map::new(env))
}

pub fn set_delegation(env: &Env, guild_id: u64, delegator: &Address, delegate: &Address) {
    let mut delegations: Map<(u64, Address), Address> = env
        .storage()
        .persistent()
        .get(&DELEGATIONS_KEY)
        .unwrap_or_else(|| Map::new(env));

    delegations.set((guild_id, delegator.clone()), delegate.clone());
    env.storage()
        .persistent()
        .set(&DELEGATIONS_KEY, &delegations);
}

pub fn remove_delegation(env: &Env, guild_id: u64, delegator: &Address) {
    let mut delegations: Map<(u64, Address), Address> = env
        .storage()
        .persistent()
        .get(&DELEGATIONS_KEY)
        .unwrap_or_else(|| Map::new(env));

    if delegations.contains_key((guild_id, delegator.clone())) {
        delegations.remove((guild_id, delegator.clone()));
        env.storage()
            .persistent()
            .set(&DELEGATIONS_KEY, &delegations);
    }
}

pub fn get_delegate(env: &Env, guild_id: u64, delegator: &Address) -> Option<Address> {
    let delegations: Map<(u64, Address), Address> = env
        .storage()
        .persistent()
        .get(&DELEGATIONS_KEY)
        .unwrap_or_else(|| Map::new(env));

    delegations.get((guild_id, delegator.clone()))
}

pub fn get_config(env: &Env, guild_id: u64) -> GovernanceConfig {
    let configs: Map<u64, GovernanceConfig> = env
        .storage()
        .persistent()
        .get(&GOV_CONFIG_KEY)
        .unwrap_or_else(|| Map::new(env));

    configs
        .get(guild_id)
        .unwrap_or_else(GovernanceConfig::default)
}

pub fn set_config(env: &Env, guild_id: u64, config: &GovernanceConfig) {
    let mut configs: Map<u64, GovernanceConfig> = env
        .storage()
        .persistent()
        .get(&GOV_CONFIG_KEY)
        .unwrap_or_else(|| Map::new(env));

    configs.set(guild_id, config.clone());
    env.storage().persistent().set(&GOV_CONFIG_KEY, &configs);
}

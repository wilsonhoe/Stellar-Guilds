#[cfg(test)]
mod tests {
    use crate::governance::types::{ProposalStatus, ProposalType, VoteDecision};
    use crate::guild::types::Role;
    use crate::StellarGuildsContract;
    use crate::StellarGuildsContractClient;
    use soroban_sdk::testutils::{Address as _, Ledger, LedgerInfo};
    use soroban_sdk::{Address, Env, String};

    fn setup_env() -> Env {
        let env = Env::default();
        env.budget().reset_unlimited();
        env
    }

    fn set_ledger_timestamp(env: &Env, timestamp: u64) {
        env.ledger().set(LedgerInfo {
            timestamp,
            protocol_version: 20,
            sequence_number: 0,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 100,
            min_persistent_entry_ttl: 100,
            max_entry_ttl: 1000000,
        });
    }

    fn register_and_init_contract(env: &Env) -> Address {
        let contract_id = env.register_contract(None, StellarGuildsContract);
        let client = StellarGuildsContractClient::new(env, &contract_id);
        client.initialize(&Address::generate(&env));
        contract_id
    }

    fn setup_guild(client: &StellarGuildsContractClient<'_>, env: &Env, owner: &Address) -> u64 {
        let name = String::from_str(env, "Gov Guild");
        let desc = String::from_str(env, "Governance test guild");
        client.create_guild(&name, &desc, owner)
    }

    fn setup_guild_with_members(
        env: &Env,
        client: &StellarGuildsContractClient<'_>,
        owner: &Address,
    ) -> (u64, Address, Address, Address) {
        let admin = Address::generate(&env);
        let member = Address::generate(&env);
        let contributor = Address::generate(&env);

        let guild_id = setup_guild(client, env, owner);

        client.add_member(&guild_id, &admin, &Role::Admin, owner);
        client.add_member(&guild_id, &member, &Role::Member, owner);
        client.add_member(&guild_id, &contributor, &Role::Contributor, owner);

        (guild_id, admin, member, contributor)
    }

    #[test]
    fn test_create_proposal_basic() {
        let env = setup_env();
        let owner = Address::generate(&env);

        set_ledger_timestamp(&env, 1000);
        env.mock_all_auths();

        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);

        let (guild_id, _admin, _member, _contributor) =
            setup_guild_with_members(&env, &client, &owner);

        let proposal_id = client.create_proposal(
            &guild_id,
            &owner,
            &ProposalType::GeneralDecision,
            &String::from_str(&env, "Test Proposal"),
            &String::from_str(&env, "Description"),
        );

        assert_eq!(proposal_id, 1);

        let proposal = client.get_proposal(&proposal_id);
        assert_eq!(proposal.guild_id, guild_id);
        assert_eq!(proposal.proposer, owner);
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[test]
    fn test_vote_weights_and_execution() {
        let env = setup_env();
        let owner = Address::generate(&env);

        set_ledger_timestamp(&env, 1000);
        env.mock_all_auths();

        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);

        let (guild_id, admin, member, contributor) =
            setup_guild_with_members(&env, &client, &owner);

        let proposal_id = client.create_proposal(
            &guild_id,
            &owner,
            &ProposalType::GeneralDecision,
            &String::from_str(&env, "Test Proposal"),
            &String::from_str(&env, "Description"),
        );

        client.vote(&proposal_id, &owner, &VoteDecision::For);
        client.vote(&proposal_id, &admin, &VoteDecision::For);
        client.vote(&proposal_id, &member, &VoteDecision::Against);
        client.vote(&proposal_id, &contributor, &VoteDecision::Abstain);

        let proposal = client.get_proposal(&proposal_id);
        let end = proposal.voting_end;
        set_ledger_timestamp(&env, end + 1);

        let status = client.finalize_proposal(&proposal_id);
        assert_eq!(status, ProposalStatus::Passed);

        let proposal_after_finalize = client.get_proposal(&proposal_id);
        assert_eq!(proposal_after_finalize.votes_for, 15);
        assert_eq!(proposal_after_finalize.votes_against, 2);
        assert_eq!(proposal_after_finalize.votes_abstain, 1);

        // Ensure proper execution utilizing the new auth executor paradigm
        let is_executed = client.execute_proposal(&proposal_id, &owner);
        assert!(is_executed);

        let final_proposal = client.get_proposal(&proposal_id);
        assert_eq!(final_proposal.status, ProposalStatus::Executed);
    }

    #[test]
    fn test_vote_delegation_and_execution() {
        let env = setup_env();
        let owner = Address::generate(&env);

        set_ledger_timestamp(&env, 1000);
        env.mock_all_auths();

        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);

        let (guild_id, admin, member, contributor) =
            setup_guild_with_members(&env, &client, &owner);

        let proposal_id = client.create_proposal(
            &guild_id,
            &owner,
            &ProposalType::GeneralDecision,
            &String::from_str(&env, "Delegation Proposal"),
            &String::from_str(&env, "Delegation"),
        );

        client.delegate_vote(&guild_id, &member, &admin);
        client.delegate_vote(&guild_id, &contributor, &member);

        client.vote(&proposal_id, &admin, &VoteDecision::For);

        let proposal = client.get_proposal(&proposal_id);
        let end = proposal.voting_end;
        set_ledger_timestamp(&env, end + 1);

        let status = client.finalize_proposal(&proposal_id);
        assert_eq!(status, ProposalStatus::Passed);

        let proposal_after_finalize = client.get_proposal(&proposal_id);
        assert_eq!(proposal_after_finalize.votes_for, 8);

        // Execute to prove lifecycle completion
        let is_executed = client.execute_proposal(&proposal_id, &admin);
        assert!(is_executed);
    }

    #[test]
    #[should_panic(expected = "only passed proposals can be executed")]
    fn test_quorum_rejection_prevents_execution() {
        let env = setup_env();
        let owner = Address::generate(&env);

        set_ledger_timestamp(&env, 1000);
        env.mock_all_auths();

        let contract_id = register_and_init_contract(&env);
        let client = StellarGuildsContractClient::new(&env, &contract_id);

        let (guild_id, _admin, _member, contributor) =
            setup_guild_with_members(&env, &client, &owner);

        let proposal_id = client.create_proposal(
            &guild_id,
            &owner,
            &ProposalType::GeneralDecision,
            &String::from_str(&env, "Low Quorum"),
            &String::from_str(&env, "Low quorum"),
        );

        client.vote(&proposal_id, &contributor, &VoteDecision::For);

        let proposal = client.get_proposal(&proposal_id);
        let end = proposal.voting_end;
        set_ledger_timestamp(&env, end + 1);

        let status = client.finalize_proposal(&proposal_id);
        assert_eq!(status, ProposalStatus::Rejected);

        // Should panic since it didn't pass quorum
        client.execute_proposal(&proposal_id, &owner);
    }
}

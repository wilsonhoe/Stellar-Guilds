#![cfg(test)]

use crate::{StellarGuildsContract, StellarGuildsContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarGuildsContract);
    let client = StellarGuildsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    
    // First initialization should succeed
    assert!(client.initialize(&admin));
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_initialize_twice_panics() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarGuildsContract);
    let client = StellarGuildsContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    
    client.initialize(&admin);
    
    // Second initialization should panic
    client.initialize(&admin);
}

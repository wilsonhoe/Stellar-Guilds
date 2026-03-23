use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{Address, Env};

/// Transfer funds from funder to contract
pub fn lock_funds(env: &Env, token: &Address, funder: &Address, amount: i128) {
    let client = TokenClient::new(env, token);
    // Transfer tokens from funder to this contract
    client.transfer(funder, &env.current_contract_address(), &amount);
}

/// Release funds from contract to recipient (claimer or original funder)
pub fn release_funds(env: &Env, token: &Address, recipient: &Address, amount: i128) {
    let client = TokenClient::new(env, token);
    // Transfer tokens from this contract to recipient
    client.transfer(&env.current_contract_address(), recipient, &amount);
}

#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

// Test 1 (Happy path): The MVP transaction executes successfully end-to-end
#[test]
fn test_happy_path_mint_and_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, BlockTagContract);
    let client = BlockTagContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let cert_id = BytesN::from_array(&env, &[1; 32]);
    let uri = String::from_str(&env, "ipfs://sneaker-metadata");

    client.init(&admin);
    client.mint(&admin, &seller, &cert_id, &uri);

    // Verify seller holds the asset before meetup
    assert_eq!(client.get_owner(&cert_id), seller);

    // Transfer asset during cash exchange
    client.transfer(&seller, &buyer, &cert_id);

    // Verify buyer now holds the asset
    assert_eq!(client.get_owner(&cert_id), buyer);
}

// Test 2 (Edge case): Unauthorized caller tries to mint
#[test]
#[should_panic(expected = "Only the authenticator can mint")]
fn test_unauthorized_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BlockTagContract);
    let client = BlockTagContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fake_admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let cert_id = BytesN::from_array(&env, &[2; 32]);
    let uri = String::from_str(&env, "ipfs://fake-metadata");

    client.init(&admin);
    // Fake admin attempts to mint
    client.mint(&fake_admin, &seller, &cert_id, &uri); 
}

// Test 3 (Edge case): Transferring a certificate that doesn't exist
#[test]
#[should_panic(expected = "Certificate not found")]
fn test_transfer_nonexistent_certificate() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BlockTagContract);
    let client = BlockTagContractClient::new(&env, &contract_id);

    let from = Address::generate(&env);
    let to = Address::generate(&env);
    let cert_id = BytesN::from_array(&env, &[3; 32]);

    client.transfer(&from, &to, &cert_id);
}

// Test 4 (Edge case): Unauthorized user attempts to transfer someone else's certificate
#[test]
#[should_panic(expected = "Not the authorized owner")]
fn test_unauthorized_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BlockTagContract);
    let client = BlockTagContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let thief = Address::generate(&env);
    let buyer = Address::generate(&env);
    let cert_id = BytesN::from_array(&env, &[4; 32]);
    let uri = String::from_str(&env, "ipfs://sneaker-metadata");

    client.init(&admin);
    client.mint(&admin, &seller, &cert_id, &uri);

    // Thief attempts to transfer seller's item
    client.transfer(&thief, &buyer, &cert_id);
}

// Test 5 (State verification): Verify state reflects correctly after MVP initialization & mint
#[test]
fn test_state_verification_after_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BlockTagContract);
    let client = BlockTagContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let cert_id = BytesN::from_array(&env, &[5; 32]);
    let uri = String::from_str(&env, "ipfs://watch-metadata");

    client.init(&admin);
    client.mint(&admin, &seller, &cert_id, &uri);

    // Read the contract state utilizing public get_owner 
    let owner = client.get_owner(&cert_id);
    assert_eq!(owner, seller);
}
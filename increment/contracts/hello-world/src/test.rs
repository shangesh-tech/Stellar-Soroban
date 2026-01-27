#![cfg(test)] // This is a compiler directive that tells Rust: "Only compile this file when running tests." It won't be included in the final build.
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

#[test]
fn test_make_count_by_1() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    assert_eq!(client.make_count_by_1(), 1);
    assert_eq!(client.make_count_by_1(), 2);
    assert_eq!(client.make_count_by_1(), 3);
}

#[test]
fn test_make_count_by_value() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    assert_eq!(client.make_count_by_value(5), 5);
    assert_eq!(client.make_count_by_value(10), 15);
    assert_eq!(client.make_count_by_value(20), 35);
}

#[test]
fn test_reset() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    client.reset();
    assert_eq!(client.increment(), 1);
}

#[test]
fn test_get_count() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    assert_eq!(client.get_count(), 0);
    client.make_count_by_1();
    assert_eq!(client.get_count(), 1);
    client.make_count_by_value(4);
    assert_eq!(client.get_count(), 5);
}
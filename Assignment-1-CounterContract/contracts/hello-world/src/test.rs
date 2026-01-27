#![cfg(test)]

use crate::{CounterContract, CounterContractClient};
use soroban_sdk::Env;

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // Initial count should be 0
    assert_eq!(client.get_count(), 0);
    
    // Test incrementing
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    
    // Verify final count
    assert_eq!(client.get_count(), 3);
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // First increment a few times
    client.increment();
    client.increment();
    client.increment();
    assert_eq!(client.get_count(), 3);
    
    // Now decrement
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    
    // Decrement at 0 should stay at 0 (no underflow)
    assert_eq!(client.decrement(), 0);
}

#[test]
fn test_add() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // Note: In Soroban SDK, client methods take references for primitive types
    assert_eq!(client.add(&5), 5);
    assert_eq!(client.add(&10), 15);
    assert_eq!(client.add(&20), 35);
    
    // Verify final count
    assert_eq!(client.get_count(), 35);
}

#[test]
fn test_reset() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // Increment a few times
    client.increment();
    client.increment();
    assert_eq!(client.get_count(), 2);
    
    // Reset
    client.reset();
    assert_eq!(client.get_count(), 0);
    
    // Increment again after reset
    assert_eq!(client.increment(), 1);
}

#[test]
fn test_get_count_initial() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // Initial count should be 0
    assert_eq!(client.get_count(), 0);
}

#[test]
fn test_combined_operations() {
    let env = Env::default();
    let contract_id = env.register(CounterContract, ());
    let client = CounterContractClient::new(&env, &contract_id);
    
    // Complex sequence of operations
    client.increment();           // 1
    client.increment();           // 2
    client.add(&8);               // 10
    client.decrement();           // 9
    client.add(&1);               // 10
    
    assert_eq!(client.get_count(), 10);
    
    client.reset();               // 0
    assert_eq!(client.get_count(), 0);
}
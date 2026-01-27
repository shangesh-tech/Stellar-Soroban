#![cfg(test)]

use crate::{DataTypesContract, DataTypesContractClient, DataRecord, DataTypeError};
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Bytes, Env, String, Symbol};

// ═══════════════════════════════════════════════════════════════════════════════
//                          PRIMITIVE INTEGER TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_u32() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Initial value should be 0
    assert_eq!(client.get_u32(), 0);
    
    // Set and verify
    client.set_u32(&42);
    assert_eq!(client.get_u32(), 42);
    
    // Test max value
    client.set_u32(&u32::MAX);
    assert_eq!(client.get_u32(), u32::MAX);
}

#[test]
fn test_i32() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Test positive
    client.set_i32(&100);
    assert_eq!(client.get_i32(), 100);
    
    // Test negative (unlike u32, i32 can be negative!)
    client.set_i32(&-100);
    assert_eq!(client.get_i32(), -100);
    
    // Test min/max
    client.set_i32(&i32::MAX);
    assert_eq!(client.get_i32(), i32::MAX);
    
    client.set_i32(&i32::MIN);
    assert_eq!(client.get_i32(), i32::MIN);
}

#[test]
fn test_u64() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Test large number
    let large_num: u64 = 18_446_744_073_709_551_000;
    client.set_u64(&large_num);
    assert_eq!(client.get_u64(), large_num);
}

#[test]
fn test_i64() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.set_i64(&-9_223_372_036_854_775_000);
    assert_eq!(client.get_i64(), -9_223_372_036_854_775_000);
}

#[test]
fn test_u128() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // u128 is great for very large numbers
    let huge_num: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_000;
    client.set_u128(&huge_num);
    assert_eq!(client.get_u128(), huge_num);
}

#[test]
fn test_i128_token_amount() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // i128 is the standard type for token amounts in Soroban!
    // Example: 1000 tokens with 7 decimals = 10_000_000_000 (10 billion)
    let token_amount: i128 = 10_000_000_000;
    client.set_i128(&token_amount);
    assert_eq!(client.get_i128(), token_amount);
    
    // Can also be negative (for debits/transfers)
    client.set_i128(&-5_000_000_000);
    assert_eq!(client.get_i128(), -5_000_000_000);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                          BOOLEAN TEST
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_bool() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Default is false
    assert_eq!(client.get_bool(), false);
    
    // Set to true
    client.set_bool(&true);
    assert_eq!(client.get_bool(), true);
    
    // Set back to false
    client.set_bool(&false);
    assert_eq!(client.get_bool(), false);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                          STRING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_string() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create and store a string
    let hello = String::from_str(&env, "Hello, Soroban!");
    client.set_string(&hello);
    
    // Retrieve and verify
    let stored = client.get_string();
    assert_eq!(stored, hello);
}

#[test]
fn test_string_concat() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    let str1 = String::from_str(&env, "Hello, ");
    let str2 = String::from_str(&env, "World!");
    
    let result = client.concat_strings(&str1, &str2);
    let expected = String::from_str(&env, "Hello, World!");
    
    assert_eq!(result, expected);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                          BYTES TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_bytes() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create bytes from array
    let data = Bytes::from_array(&env, &[0x01, 0x02, 0x03, 0x04, 0x05]);
    client.set_bytes(&data);
    
    // Verify storage
    let stored = client.get_bytes();
    assert_eq!(stored, data);
    
    // Verify length
    assert_eq!(client.get_bytes_length(), 5);
}

#[test]
fn test_fixed_bytes() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create fixed 32-byte array (like bytes32 in Solidity)
    let fixed_bytes = client.create_fixed_bytes();
    
    // Fixed bytes should be exactly 32 bytes
    assert_eq!(fixed_bytes.len(), 32);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                          VECTOR TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_vector_operations() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Initialize empty vector
    client.init_vector();
    assert_eq!(client.vector_len(), 0);
    
    // Push values
    client.vector_push(&100);
    client.vector_push(&200);
    client.vector_push(&300);
    
    // Check length
    assert_eq!(client.vector_len(), 3);
    
    // Get by index - client returns the Result's Ok value directly, panics on Err
    assert_eq!(client.vector_get(&0), 100);
    assert_eq!(client.vector_get(&1), 200);
    assert_eq!(client.vector_get(&2), 300);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]  // InvalidIndex = 2
fn test_vector_get_invalid_index() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_vector();
    // This should panic because index 99 doesn't exist
    let _ = client.vector_get(&99);
}

#[test]
fn test_vector_pop() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_vector();
    client.vector_push(&10);
    client.vector_push(&20);
    
    // Pop should return last element
    assert_eq!(client.vector_pop(), 20);
    assert_eq!(client.vector_len(), 1);
    
    assert_eq!(client.vector_pop(), 10);
    assert_eq!(client.vector_len(), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]  // EmptyVector = 4
fn test_vector_pop_empty() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_vector();
    // Pop on empty should panic
    let _ = client.vector_pop();
}

#[test]
fn test_vector_set() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_vector();
    client.vector_push(&1);
    client.vector_push(&2);
    client.vector_push(&3);
    
    // Modify middle element
    client.vector_set(&1, &999);
    
    assert_eq!(client.vector_get(&0), 1);
    assert_eq!(client.vector_get(&1), 999);  // Changed!
    assert_eq!(client.vector_get(&2), 3);
}

#[test]
fn test_vector_with_initial_values() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create vector with values using vec! macro
    let vec = client.create_vector_with_values();
    
    assert_eq!(vec.len(), 5);
    assert_eq!(vec.get(0), Some(100));
    assert_eq!(vec.get(4), Some(500));
}

// ═══════════════════════════════════════════════════════════════════════════════
//                          MAP TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_map_operations() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Initialize map
    client.init_map();
    assert_eq!(client.map_len(), 0);
    
    // Set values
    let key1: Symbol = symbol_short!("alice");
    let key2: Symbol = symbol_short!("bob");
    
    client.map_set(&key1, &1000);
    client.map_set(&key2, &2000);
    
    // Check length
    assert_eq!(client.map_len(), 2);
    
    // Get values
    assert_eq!(client.map_get(&key1), 1000);
    assert_eq!(client.map_get(&key2), 2000);
    
    // Check contains
    assert_eq!(client.map_contains(&key1), true);
    assert_eq!(client.map_contains(&symbol_short!("unknown")), false);
}

#[test]
fn test_map_remove() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_map();
    
    let key: Symbol = symbol_short!("test");
    client.map_set(&key, &999);
    
    assert_eq!(client.map_contains(&key), true);
    
    // Remove and verify - returns the removed value
    assert_eq!(client.map_remove(&key), 999);
    assert_eq!(client.map_contains(&key), false);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]  // KeyNotFound = 3
fn test_map_remove_nonexistent() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    client.init_map();
    let key: Symbol = symbol_short!("test");
    // Remove non-existent key should panic
    let _ = client.map_remove(&key);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                     ADDRESS-BASED STORAGE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_address_balance() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Generate test addresses
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    
    // Initial balances should be 0
    assert_eq!(client.get_balance(&alice), 0);
    assert_eq!(client.get_balance(&bob), 0);
    
    // Set balances
    client.set_balance(&alice, &1_000_000);
    client.set_balance(&bob, &2_500_000);
    
    // Verify
    assert_eq!(client.get_balance(&alice), 1_000_000);
    assert_eq!(client.get_balance(&bob), 2_500_000);
    
    // Update balance
    client.set_balance(&alice, &500_000);
    assert_eq!(client.get_balance(&alice), 500_000);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                     CUSTOM STRUCT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_custom_struct() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create a record
    let name = String::from_str(&env, "Test Record");
    let record = client.create_record(&1, &name, &9999, &true);
    
    assert_eq!(record.id, 1);
    assert_eq!(record.name, name);
    assert_eq!(record.value, 9999);
    assert_eq!(record.active, true);
}

#[test]
fn test_store_and_retrieve_record() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Create and store
    let name = String::from_str(&env, "Stored Record");
    let record = DataRecord {
        id: 42,
        name: name.clone(),
        value: 12345,
        active: true,
    };
    
    client.store_record(&42, &record);
    
    // Retrieve and verify - client returns value directly
    let retrieved = client.get_record(&42);
    assert_eq!(retrieved.id, 42);
    assert_eq!(retrieved.name, name);
    assert_eq!(retrieved.value, 12345);
    assert_eq!(retrieved.active, true);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]  // ValueNotFound = 1
fn test_record_not_found() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Try to get non-existent record - should panic
    let _ = client.get_record(&999);
}

// ═══════════════════════════════════════════════════════════════════════════════
//                     TYPE CONVERSION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_u32_to_i128() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // u32 to i128
    assert_eq!(client.u32_to_i128(&100), 100_i128);
    assert_eq!(client.u32_to_i128(&u32::MAX), u32::MAX as i128);
}

#[test]
fn test_i128_to_u32_success() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Valid conversion
    assert_eq!(client.i128_to_u32(&100), 100);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]  // InvalidIndex = 2
fn test_i128_to_u32_negative() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Negative number should fail
    let _ = client.i128_to_u32(&-1);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]  // InvalidIndex = 2
fn test_i128_to_u32_too_large() {
    let env = Env::default();
    let contract_id = env.register(DataTypesContract, ());
    let client = DataTypesContractClient::new(&env, &contract_id);
    
    // Too large number should fail
    let too_large: i128 = (u32::MAX as i128) + 1;
    let _ = client.i128_to_u32(&too_large);
}

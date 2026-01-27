#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    symbol_short, vec, 
    Address, Bytes, BytesN, Env, Map, String, Symbol, Vec,
};

// ═══════════════════════════════════════════════════════════════════════════════
//                              STORAGE KEYS
// ═══════════════════════════════════════════════════════════════════════════════
// Using symbol_short! for keys ≤ 9 characters

const KEY_U32: Symbol = symbol_short!("u32_val");
const KEY_I32: Symbol = symbol_short!("i32_val");
const KEY_U64: Symbol = symbol_short!("u64_val");
const KEY_I64: Symbol = symbol_short!("i64_val");
const KEY_U128: Symbol = symbol_short!("u128_val");
const KEY_I128: Symbol = symbol_short!("i128_val");
const KEY_BOOL: Symbol = symbol_short!("bool_val");
const KEY_STRING: Symbol = symbol_short!("str_val");
const KEY_BYTES: Symbol = symbol_short!("bytes");
const KEY_VEC: Symbol = symbol_short!("vec_val");
const KEY_MAP: Symbol = symbol_short!("map_val");

// TTL Constants
const DAY_IN_LEDGERS: u32 = 17_280;
const BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS;

// ═══════════════════════════════════════════════════════════════════════════════
//                              CUSTOM ERROR TYPE
// ═══════════════════════════════════════════════════════════════════════════════
// Solidity equivalent: custom errors with revert reasons

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DataTypeError {
    ValueNotFound = 1,
    InvalidIndex = 2,
    KeyNotFound = 3,
    EmptyVector = 4,
}

// ═══════════════════════════════════════════════════════════════════════════════
//                              CUSTOM STRUCT TYPE
// ═══════════════════════════════════════════════════════════════════════════════
// Solidity equivalent: struct

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataRecord {
    pub id: u32,
    pub name: String,
    pub value: i128,
    pub active: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
//                              MAIN CONTRACT
// ═══════════════════════════════════════════════════════════════════════════════

#[contract]
pub struct DataTypesContract;

#[contractimpl]
impl DataTypesContract {
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                     PRIMITIVE INTEGER TYPES
    // ═══════════════════════════════════════════════════════════════════════════
    
    // ─────────────────────────────────────────────────────────────────────────
    // u32: Unsigned 32-bit integer (0 to 4,294,967,295)
    // Solidity equivalent: uint32
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_u32(env: Env, value: u32) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_U32, &value);
    }
    
    pub fn get_u32(env: Env) -> u32 {
        env.storage().instance().get(&KEY_U32).unwrap_or(0)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // i32: Signed 32-bit integer (-2,147,483,648 to 2,147,483,647)
    // Solidity equivalent: int32
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_i32(env: Env, value: i32) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_I32, &value);
    }
    
    pub fn get_i32(env: Env) -> i32 {
        env.storage().instance().get(&KEY_I32).unwrap_or(0)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // u64: Unsigned 64-bit integer (0 to 18,446,744,073,709,551,615)
    // Solidity equivalent: uint64
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_u64(env: Env, value: u64) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_U64, &value);
    }
    
    pub fn get_u64(env: Env) -> u64 {
        env.storage().instance().get(&KEY_U64).unwrap_or(0)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // i64: Signed 64-bit integer
    // Solidity equivalent: int64
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_i64(env: Env, value: i64) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_I64, &value);
    }
    
    pub fn get_i64(env: Env) -> i64 {
        env.storage().instance().get(&KEY_I64).unwrap_or(0)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // u128: Unsigned 128-bit integer (for large numbers, token amounts)
    // Solidity equivalent: uint128
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_u128(env: Env, value: u128) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_U128, &value);
    }
    
    pub fn get_u128(env: Env) -> u128 {
        env.storage().instance().get(&KEY_U128).unwrap_or(0)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // i128: Signed 128-bit integer (commonly used for token amounts in Soroban!)
    // Solidity equivalent: int128 (but Soroban uses i128 extensively for tokens)
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn set_i128(env: Env, value: i128) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_I128, &value);
    }
    
    pub fn get_i128(env: Env) -> i128 {
        env.storage().instance().get(&KEY_I128).unwrap_or(0)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         BOOLEAN TYPE
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: bool
    
    pub fn set_bool(env: Env, value: bool) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_BOOL, &value);
    }
    
    pub fn get_bool(env: Env) -> bool {
        env.storage().instance().get(&KEY_BOOL).unwrap_or(false)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         STRING TYPE
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: string
    // NOTE: In no_std environment, we use soroban_sdk::String, not Rust's String
    
    pub fn set_string(env: Env, value: String) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_STRING, &value);
    }
    
    pub fn get_string(env: Env) -> String {
        env.storage()
            .instance()
            .get(&KEY_STRING)
            .unwrap_or(String::from_str(&env, ""))
    }
    
    /// Create a string from a static str (useful for initialization)
    pub fn create_string(env: Env, value: String) -> String {
        // In Soroban, you typically receive strings from the client
        // This just demonstrates returning a string
        value
    }
    
    /// Concatenate two strings (demonstrating string operations)
    pub fn concat_strings(env: Env, a: String, b: String) -> String {
        // Soroban strings don't have direct concat, so we use Bytes
        let bytes_a = a.to_bytes();
        let bytes_b = b.to_bytes();
        
        // Create a new bytes buffer and append both
        let mut result = Bytes::new(&env);
        result.append(&bytes_a);
        result.append(&bytes_b);
        
        // Convert Bytes to a slice for from_bytes
        let mut arr = [0u8; 256]; // buffer for concatenated string
        let len = result.len() as usize;
        for i in 0..len {
            arr[i] = result.get(i as u32).unwrap();
        }
        String::from_bytes(&env, &arr[..len])
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         BYTES TYPE
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: bytes (dynamic) and bytes32 (fixed)
    
    /// Set dynamic bytes (like Solidity's `bytes`)
    pub fn set_bytes(env: Env, value: Bytes) {
        Self::extend_ttl(&env);
        env.storage().instance().set(&KEY_BYTES, &value);
    }
    
    pub fn get_bytes(env: Env) -> Bytes {
        env.storage()
            .instance()
            .get(&KEY_BYTES)
            .unwrap_or(Bytes::new(&env))
    }
    
    /// Get the length of stored bytes
    pub fn get_bytes_length(env: Env) -> u32 {
        let bytes: Bytes = env.storage()
            .instance()
            .get(&KEY_BYTES)
            .unwrap_or(Bytes::new(&env));
        bytes.len()
    }
    
    /// Create bytes from array - demonstrates BytesN (fixed-size bytes)
    /// Solidity equivalent: bytes32
    pub fn create_fixed_bytes(env: Env) -> BytesN<32> {
        // Create a 32-byte array (like bytes32 in Solidity)
        BytesN::from_array(&env, &[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
            0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
            0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
        ])
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         VECTOR TYPE (Dynamic Array)
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: uint256[] (dynamic array)
    
    /// Initialize an empty vector
    pub fn init_vector(env: Env) {
        Self::extend_ttl(&env);
        let empty_vec: Vec<i128> = Vec::new(&env);
        env.storage().instance().set(&KEY_VEC, &empty_vec);
    }
    
    /// Push a value to the end of the vector
    pub fn vector_push(env: Env, value: i128) {
        Self::extend_ttl(&env);
        let mut vec: Vec<i128> = env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env));
        
        vec.push_back(value);
        env.storage().instance().set(&KEY_VEC, &vec);
    }
    
    /// Pop the last value from the vector
    pub fn vector_pop(env: Env) -> Result<i128, DataTypeError> {
        Self::extend_ttl(&env);
        let mut vec: Vec<i128> = env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env));
        
        match vec.pop_back() {
            Some(value) => {
                env.storage().instance().set(&KEY_VEC, &vec);
                Ok(value)
            }
            None => Err(DataTypeError::EmptyVector),
        }
    }
    
    /// Get value at specific index
    pub fn vector_get(env: Env, index: u32) -> Result<i128, DataTypeError> {
        let vec: Vec<i128> = env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env));
        
        vec.get(index).ok_or(DataTypeError::InvalidIndex)
    }
    
    /// Get vector length
    pub fn vector_len(env: Env) -> u32 {
        let vec: Vec<i128> = env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env));
        vec.len()
    }
    
    /// Get entire vector
    pub fn vector_get_all(env: Env) -> Vec<i128> {
        env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env))
    }
    
    /// Set value at specific index
    pub fn vector_set(env: Env, index: u32, value: i128) -> Result<(), DataTypeError> {
        Self::extend_ttl(&env);
        let mut vec: Vec<i128> = env.storage()
            .instance()
            .get(&KEY_VEC)
            .unwrap_or(Vec::new(&env));
        
        if index >= vec.len() {
            return Err(DataTypeError::InvalidIndex);
        }
        
        vec.set(index, value);
        env.storage().instance().set(&KEY_VEC, &vec);
        Ok(())
    }
    
    /// Create vector with initial values (like array literal in Solidity)
    pub fn create_vector_with_values(env: Env) -> Vec<i128> {
        // Using vec! macro - similar to [1, 2, 3, 4, 5] in Solidity
        vec![&env, 100, 200, 300, 400, 500]
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         MAP TYPE (Key-Value Store)
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: mapping(address => uint256)
    
    /// Initialize an empty map
    pub fn init_map(env: Env) {
        Self::extend_ttl(&env);
        let empty_map: Map<Symbol, i128> = Map::new(&env);
        env.storage().instance().set(&KEY_MAP, &empty_map);
    }
    
    /// Set a value in the map
    pub fn map_set(env: Env, key: Symbol, value: i128) {
        Self::extend_ttl(&env);
        let mut map: Map<Symbol, i128> = env.storage()
            .instance()
            .get(&KEY_MAP)
            .unwrap_or(Map::new(&env));
        
        map.set(key, value);
        env.storage().instance().set(&KEY_MAP, &map);
    }
    
    /// Get a value from the map
    pub fn map_get(env: Env, key: Symbol) -> Result<i128, DataTypeError> {
        let map: Map<Symbol, i128> = env.storage()
            .instance()
            .get(&KEY_MAP)
            .unwrap_or(Map::new(&env));
        
        map.get(key).ok_or(DataTypeError::KeyNotFound)
    }
    
    /// Check if key exists in map
    pub fn map_contains(env: Env, key: Symbol) -> bool {
        let map: Map<Symbol, i128> = env.storage()
            .instance()
            .get(&KEY_MAP)
            .unwrap_or(Map::new(&env));
        
        map.contains_key(key)
    }
    
    /// Remove a key from the map
    pub fn map_remove(env: Env, key: Symbol) -> Result<i128, DataTypeError> {
        Self::extend_ttl(&env);
        let mut map: Map<Symbol, i128> = env.storage()
            .instance()
            .get(&KEY_MAP)
            .unwrap_or(Map::new(&env));
        
        // First get the value before removing
        let value = map.get(key.clone()).ok_or(DataTypeError::KeyNotFound)?;
        
        // Now remove the key (returns Option<()> in Soroban SDK)
        map.remove(key);
        env.storage().instance().set(&KEY_MAP, &map);
        Ok(value)
    }
    
    /// Get map size
    pub fn map_len(env: Env) -> u32 {
        let map: Map<Symbol, i128> = env.storage()
            .instance()
            .get(&KEY_MAP)
            .unwrap_or(Map::new(&env));
        map.len()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                    ADDRESS-BASED MAP (Common Pattern)
    // ═══════════════════════════════════════════════════════════════════════════
    // Solidity equivalent: mapping(address => uint256) balances
    
    /// Set balance for an address (like balances[addr] = value)
    pub fn set_balance(env: Env, addr: Address, amount: i128) {
        Self::extend_ttl(&env);
        // Using persistent storage for per-user data
        env.storage().persistent().set(&addr, &amount);
    }
    
    /// Get balance for an address (like balances[addr])
    pub fn get_balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                    CUSTOM STRUCT OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Create and return a custom struct (DataRecord)
    pub fn create_record(
        env: Env,
        id: u32,
        name: String,
        value: i128,
        active: bool,
    ) -> DataRecord {
        DataRecord {
            id,
            name,
            value,
            active,
        }
    }
    
    /// Store a record with a given key
    pub fn store_record(env: Env, key: u32, record: DataRecord) {
        Self::extend_ttl(&env);
        env.storage().persistent().set(&key, &record);
    }
    
    /// Retrieve a record by key
    pub fn get_record(env: Env, key: u32) -> Result<DataRecord, DataTypeError> {
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(DataTypeError::ValueNotFound)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                    TYPE CONVERSION UTILITIES
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Convert u32 to i128 (common for token operations)
    pub fn u32_to_i128(value: u32) -> i128 {
        value as i128
    }
    
    /// Convert i128 to u32 (with bounds checking)
    pub fn i128_to_u32(value: i128) -> Result<u32, DataTypeError> {
        if value < 0 || value > u32::MAX as i128 {
            return Err(DataTypeError::InvalidIndex);
        }
        Ok(value as u32)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //                         TTL EXTENSION HELPER
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn extend_ttl(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }
}

mod test;

/*
╔══════════════════════════════════════════════════════════════════════════════╗
║                    SOROBAN vs SOLIDITY DATA TYPES COMPARISON                 ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  INTEGERS                                                                    ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  Solidity          │  Soroban           │  Notes                             ║
║  ──────────────────┼────────────────────┼─────────────────────────────────── ║
║  uint8             │  u32 (no u8 type)  │  Soroban uses u32 as minimum       ║
║  uint32            │  u32               │  0 to 4,294,967,295                ║
║  int32             │  i32               │  Signed 32-bit                     ║
║  uint64            │  u64               │  Unsigned 64-bit                   ║
║  int64             │  i64               │  Signed 64-bit                     ║
║  uint128           │  u128              │  Unsigned 128-bit                  ║
║  int128            │  i128              │  Most common for tokens!           ║
║  uint256           │  N/A               │  Use U256 from ethnum crate        ║
║                                                                              ║
║  STRINGS & BYTES                                                             ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  string            │  String            │  From soroban_sdk                  ║
║  bytes             │  Bytes             │  Dynamic bytes                     ║
║  bytes32           │  BytesN<32>        │  Fixed-size bytes                  ║
║  bytes20           │  BytesN<20>        │  For addresses                     ║
║                                                                              ║
║  COLLECTIONS                                                                 ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  uint256[]         │  Vec<i128>         │  Dynamic array                     ║
║  mapping(K => V)   │  Map<K, V>         │  Key-value store                   ║
║  struct            │  #[contracttype]   │  Custom data structures            ║
║  enum              │  #[contracttype]   │  Enumerations                      ║
║                                                                              ║
║  SPECIAL TYPES                                                               ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  address           │  Address           │  Account/Contract identifier       ║
║  bool              │  bool              │  true/false                        ║
║  N/A               │  Symbol            │  Short identifiers (≤32 chars)     ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
*/

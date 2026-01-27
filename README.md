# 🚀 Soroban Smart Contracts Mastery

> **A comprehensive learning path to become a Soroban Pro**  
> *For developers with Solidity/Ethereum background*

---

## 📚 Table of Contents

1. [Project Structure](#project-structure)
2. [Prerequisites](#prerequisites)
3. [Quick Start](#quick-start)
4. [Solidity to Soroban Transition Guide](#solidity-to-soroban-transition-guide)
5. [Learning Path & Assignments](#learning-path--assignments)
6. [Resources](#resources)

---

## 📁 Project Structure

```text
.
├── contracts/
│   ├── hello-world/           # ✅ Completed - Starter contract
│   ├── 01-counter/            # 📝 Assignment 1
│   ├── 02-data-types/         # 📝 Assignment 2
│   ├── 03-custom-types/       # 📝 Assignment 3
│   ├── 04-storage-patterns/   # 📝 Assignment 4
│   ├── 05-token-contract/     # 📝 Assignment 5
│   ├── 06-auth-advanced/      # 📝 Assignment 6
│   ├── 07-cross-contract/     # 📝 Assignment 7
│   ├── 08-timelock/           # 📝 Assignment 8
│   ├── 09-crowdfunding/       # 📝 Assignment 9
│   ├── 10-nft-contract/       # 📝 Assignment 10
│   ├── 11-dex-amm/            # 📝 Assignment 11
│   ├── 12-governance/         # 📝 Assignment 12
│   └── 13-upgradeable/        # 📝 Assignment 13
├── Cargo.toml
└── README.md
```

---

## ⚡ Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt

# Verify installation
stellar --version
```

---

## 🚀 Quick Start

```bash
# Build all contracts
stellar contract build

# Run tests
cargo test

# Deploy to testnet (after building)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

---

## 🔄 Solidity to Soroban Transition Guide

| Solidity Concept | Soroban Equivalent | Key Differences |
|-----------------|-------------------|-----------------|
| `contract` | `#[contract]` + `struct` | Rust struct with macro |
| `public function` | `pub fn` in `#[contractimpl]` | Explicit visibility |
| `msg.sender` | `env.current_contract_address()` | Different context model |
| `require()` | `panic!()` or `assert!()` | Rust idioms |
| `mapping` | `Map<K, V>` | Different storage semantics |
| `modifier` | Helper functions + checks | No native modifiers |
| `event` | `env.events().publish()` | Explicit publishing |
| `payable` | Native token handling | Different token model |
| `view/pure` | All functions can read | No gas for reads in same way |
| `storage slots` | Temporary/Persistent/Instance | 3 storage types |
| `constructor` | `__constructor` function | Called once on deploy |
| `selfdestruct` | Not available | Contracts persist |
| `abi.encode` | Soroban SDK types | Native serialization |

### Key Mindset Shifts

1. **No EVM**: Soroban runs on WebAssembly (WASM), not EVM
2. **Rust First**: Learn Rust patterns (ownership, borrowing, lifetimes)
3. **Storage Model**: Three tiers - Temporary, Persistent, Instance
4. **Authorization**: Explicit `require_auth()` pattern
5. **No Gas in Same Way**: Resource model differs from Ethereum
6. **XDR Types**: Stellar uses XDR for serialization

---

## 📋 Learning Path & Assignments

### 🌱 **PHASE 1: Foundations (Week 1-2)**

---

#### 📝 Assignment 1: Counter Contract
**Difficulty**: ⭐ Easy  
**Folder**: `contracts/01-counter/`  
**Solidity Equivalent**: Basic state variable + increment/decrement

**Requirements**:
- Create a counter that stores a `u32` value
- Implement `increment()` function
- Implement `decrement()` function  
- Implement `get_count()` function
- Implement `reset()` function
- Add proper tests for all functions

**Learning Goals**:
- [ ] Understanding `#[contract]` and `#[contractimpl]` macros
- [ ] Working with `Env` environment
- [ ] Basic storage with `env.storage().instance()`
- [ ] Writing and running tests

**Hints**:
```rust
// Storage keys pattern
const COUNT: Symbol = symbol_short!("COUNT");

// Getting/Setting storage
env.storage().instance().get(&COUNT).unwrap_or(0)
env.storage().instance().set(&COUNT, &new_value);
```

---

#### 📝 Assignment 2: Data Types Explorer
**Difficulty**: ⭐ Easy  
**Folder**: `contracts/02-data-types/`  
**Solidity Equivalent**: Understanding uint, int, bytes, string, arrays

**Requirements**:
- Store and retrieve different primitive types: `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
- Work with `String` type
- Work with `Bytes` type
- Work with `Vec<T>` (dynamic arrays)
- Work with `Map<K, V>`
- Implement functions to set/get each type

**Learning Goals**:
- [ ] Soroban SDK primitive types
- [ ] String handling in no_std environment
- [ ] Vector operations
- [ ] Map operations
- [ ] Type conversions

---

#### 📝 Assignment 3: Custom Types & Structs
**Difficulty**: ⭐⭐ Medium  
**Folder**: `contracts/03-custom-types/`  
**Solidity Equivalent**: Structs, Enums, custom types

**Requirements**:
- Define a `User` struct with fields: `id`, `name`, `balance`, `active`
- Define a `Status` enum: `Pending`, `Active`, `Suspended`, `Closed`
- Create CRUD operations for users
- Store users in a Map
- Implement filtering/querying users by status

**Learning Goals**:
- [ ] `#[contracttype]` macro for structs
- [ ] `#[contracterror]` for custom errors
- [ ] Enum handling in Soroban
- [ ] Complex data structure storage

**Hints**:
```rust
#[contracttype]
#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub balance: i128,
    pub status: Status,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum Status {
    Pending,
    Active,
    Suspended,
}
```

---

### 🌿 **PHASE 2: Intermediate Concepts (Week 3-4)**

---

#### 📝 Assignment 4: Storage Patterns Deep Dive
**Difficulty**: ⭐⭐ Medium  
**Folder**: `contracts/04-storage-patterns/`  
**Solidity Equivalent**: Understanding storage slots, gas optimization

**Requirements**:
- Implement contract using all 3 storage types:
  - `Temporary` storage (session-like data, expires after TTL)
  - `Persistent` storage (long-term data, rent fees)
  - `Instance` storage (contract-wide settings)
- Implement TTL (Time-To-Live) management
- Create a session-based voting system demonstrating storage differences

**Learning Goals**:
- [ ] When to use each storage type
- [ ] Cost implications of each storage type
- [ ] TTL management with `extend_ttl()`
- [ ] Storage rent model understanding

**Hints**:
```rust
// Temporary - expires, no rent
env.storage().temporary().set(&key, &value);
env.storage().temporary().extend_ttl(&key, 100, 100);

// Persistent - requires rent
env.storage().persistent().set(&key, &value);
env.storage().persistent().extend_ttl(&key, 17280, 17280);

// Instance - for contract metadata
env.storage().instance().set(&key, &value);
```

---

#### 📝 Assignment 5: Token Contract (SEP-41)
**Difficulty**: ⭐⭐⭐ Medium-Hard  
**Folder**: `contracts/05-token-contract/`  
**Solidity Equivalent**: ERC-20 Token Standard

**Requirements**:
- Implement the Stellar SEP-41 Token Interface
- Functions: `initialize`, `mint`, `burn`, `transfer`, `approve`, `transfer_from`
- Functions: `balance`, `allowance`, `decimals`, `name`, `symbol`
- Implement proper authorization checks
- Emit events for all state changes

**Learning Goals**:
- [ ] SEP-41 Token Standard (Stellar's ERC-20 equivalent)
- [ ] `require_auth()` for authorization
- [ ] Event emission with `env.events().publish()`
- [ ] Interface implementation patterns

**Hints**:
```rust
// Authorization
from.require_auth();

// Events
env.events().publish((symbol_short!("transfer"),), (from, to, amount));

// Token interface
pub trait TokenInterface {
    fn balance(env: Env, id: Address) -> i128;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    // ... more functions
}
```

---

#### 📝 Assignment 6: Advanced Authorization
**Difficulty**: ⭐⭐⭐ Hard  
**Folder**: `contracts/06-auth-advanced/`  
**Solidity Equivalent**: Access Control, Role-Based Permissions, Multi-sig

**Requirements**:
- Implement role-based access control (Admin, Operator, User)
- Create a multi-signature authorization system (N of M signatures)
- Implement time-locked operations requiring admin approval
- Sub-contract invocation with authorization

**Learning Goals**:
- [ ] `Address` type and its capabilities
- [ ] `require_auth()` vs `require_auth_for_args()`
- [ ] Multi-party authorization patterns
- [ ] Authorization context and sub-invocations

**Hints**:
```rust
// Require auth from specific address
admin.require_auth();

// Require auth with specific arguments
signer.require_auth_for_args((&amount,).into_val(&env));

// Check current invoker
env.current_contract_address()
```

---

### 🌲 **PHASE 3: Advanced Patterns (Week 5-6)**

---

#### 📝 Assignment 7: Cross-Contract Calls
**Difficulty**: ⭐⭐⭐ Hard  
**Folder**: `contracts/07-cross-contract/`  
**Solidity Equivalent**: Contract-to-contract calls, interfaces

**Requirements**:
- Create a "Factory" contract that deploys other contracts
- Create a "Registry" contract that stores deployed contract addresses
- Implement cross-contract function calls
- Pass authorization context between contracts
- Handle errors from external contract calls

**Learning Goals**:
- [ ] Contract client generation
- [ ] Deploying contracts from contracts
- [ ] Cross-contract authorization flow
- [ ] Error handling in cross-contract calls

**Hints**:
```rust
// Import another contract's client
mod other_contract {
    soroban_sdk::contractimport!(file = "../other/target/.../other.wasm");
}

// Call another contract
let client = other_contract::Client::new(&env, &contract_id);
client.some_function(&arg1, &arg2);
```

---

#### 📝 Assignment 8: Timelock Contract
**Difficulty**: ⭐⭐⭐ Hard  
**Folder**: `contracts/08-timelock/`  
**Solidity Equivalent**: TimelockController, Vesting contracts

**Requirements**:
- Create a token timelock/vesting contract
- Implement cliff period (no withdrawal before cliff)
- Implement linear vesting after cliff
- Support multiple beneficiaries
- Allow admin to revoke unvested tokens
- Calculate claimable amount based on time

**Learning Goals**:
- [ ] Working with timestamps via `env.ledger().timestamp()`
- [ ] Time-based calculations in contracts
- [ ] Vesting schedule implementation
- [ ] Token integration (use your Assignment 5 token)

**Hints**:
```rust
// Get current ledger timestamp
let now = env.ledger().timestamp();

// Get current ledger sequence
let sequence = env.ledger().sequence();
```

---

#### 📝 Assignment 9: Crowdfunding Platform
**Difficulty**: ⭐⭐⭐⭐ Hard  
**Folder**: `contracts/09-crowdfunding/`  
**Solidity Equivalent**: Kickstarter-style crowdfunding, escrow

**Requirements**:
- Create campaigns with funding goal, deadline, and creator
- Accept contributions from multiple users
- Refund all contributors if goal not met by deadline
- Transfer funds to creator if goal is met
- Track contribution history per user
- Implement campaign states: Open, Successful, Failed, Closed

**Learning Goals**:
- [ ] Complex state machine implementation
- [ ] Multiple storage patterns working together
- [ ] Time-based conditions
- [ ] Fund management and escrow patterns
- [ ] Refund mechanisms

---

### 🏔️ **PHASE 4: Expert Level (Week 7-8)**

---

#### 📝 Assignment 10: NFT Contract (SEP-TBD)
**Difficulty**: ⭐⭐⭐⭐ Expert  
**Folder**: `contracts/10-nft-contract/`  
**Solidity Equivalent**: ERC-721, ERC-1155

**Requirements**:
- Implement non-fungible token contract
- Mint unique tokens with metadata URI
- Implement transfer, approve, and transfer_from
- Support batch operations
- Implement royalty system for creators (on each transfer)
- Add metadata storage (on-chain or URI-based)

**Learning Goals**:
- [ ] NFT patterns in Soroban
- [ ] Unique ID generation
- [ ] Metadata handling
- [ ] Royalty calculations
- [ ] Batch operation optimization

---

#### 📝 Assignment 11: DEX AMM (UniSwap-style)
**Difficulty**: ⭐⭐⭐⭐⭐ Expert  
**Folder**: `contracts/11-dex-amm/`  
**Solidity Equivalent**: Uniswap V2 constant product AMM

**Requirements**:
- Implement constant product AMM (x * y = k)
- Create liquidity pools for token pairs
- Implement add/remove liquidity
- Implement swap with slippage protection
- Mint LP tokens for liquidity providers
- Implement fees (0.3%) going to LPs
- Price oracle functionality

**Learning Goals**:
- [ ] AMM mathematics and implementation
- [ ] Multi-token handling
- [ ] LP token minting/burning
- [ ] Slippage and price impact calculations
- [ ] Fee distribution

**Hints**:
```rust
// Constant product formula
// (reserve_a - amount_out) * (reserve_b + amount_in) = reserve_a * reserve_b

// Calculate output amount
fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    let amount_in_with_fee = amount_in * 997; // 0.3% fee
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = (reserve_in * 1000) + amount_in_with_fee;
    numerator / denominator
}
```

---

#### 📝 Assignment 12: Governance Contract
**Difficulty**: ⭐⭐⭐⭐⭐ Expert  
**Folder**: `contracts/12-governance/`  
**Solidity Equivalent**: Governor, Compound Governance

**Requirements**:
- Token-based voting power
- Proposal creation with description and calldata
- Voting period with configurable duration
- Vote delegation
- Quorum requirements
- Proposal execution after passing
- Timelock integration (use Assignment 8)

**Learning Goals**:
- [ ] Complex governance logic
- [ ] Vote counting and quorum
- [ ] Delegation patterns
- [ ] Cross-contract execution
- [ ] State machine for proposals

---

#### 📝 Assignment 13: Upgradeable Contract Pattern
**Difficulty**: ⭐⭐⭐⭐⭐ Expert  
**Folder**: `contracts/13-upgradeable/`  
**Solidity Equivalent**: Proxy patterns, UUPS

**Requirements**:
- Implement WASM code upgrade functionality
- Create upgrade authorization (multi-sig or governance)
- Maintain storage compatibility between versions
- Implement version tracking
- Add upgrade delay/timelock
- Test upgrade with state preservation

**Learning Goals**:
- [ ] `env.deployer().update_current_contract_wasm()`
- [ ] Storage layout considerations
- [ ] Upgrade authorization patterns
- [ ] Version management
- [ ] Testing upgrades

**Hints**:
```rust
// Upgrade contract WASM
env.deployer().update_current_contract_wasm(new_wasm_hash);
```

---

## 🎯 Bonus Challenges

After completing all assignments, challenge yourself with these:

### 🏆 Challenge 1: Yield Farming Protocol
Combine DEX, Token, and Governance to create a yield farming system.

### 🏆 Challenge 2: Lending Protocol
Implement a basic lending/borrowing protocol with collateralization.

### 🏆 Challenge 3: Bridge Contract
Create a simple bridge contract for cross-chain token transfers (mock).

### 🏆 Challenge 4: Oracle Integration
Build a price oracle and integrate it with your DEX.

---

## ✅ Progress Tracker

| # | Assignment | Difficulty | Status | Date Completed |
|---|-----------|------------|--------|----------------|
| 0 | Hello World | ⭐ | ✅ Completed | - |
| 1 | Counter | ⭐ | ⬜ Not Started | |
| 2 | Data Types | ⭐ | ⬜ Not Started | |
| 3 | Custom Types | ⭐⭐ | ⬜ Not Started | |
| 4 | Storage Patterns | ⭐⭐ | ⬜ Not Started | |
| 5 | Token (SEP-41) | ⭐⭐⭐ | ⬜ Not Started | |
| 6 | Auth Advanced | ⭐⭐⭐ | ⬜ Not Started | |
| 7 | Cross-Contract | ⭐⭐⭐ | ⬜ Not Started | |
| 8 | Timelock | ⭐⭐⭐ | ⬜ Not Started | |
| 9 | Crowdfunding | ⭐⭐⭐⭐ | ⬜ Not Started | |
| 10 | NFT Contract | ⭐⭐⭐⭐ | ⬜ Not Started | |
| 11 | DEX AMM | ⭐⭐⭐⭐⭐ | ⬜ Not Started | |
| 12 | Governance | ⭐⭐⭐⭐⭐ | ⬜ Not Started | |
| 13 | Upgradeable | ⭐⭐⭐⭐⭐ | ⬜ Not Started | |

---

## 📖 Resources

### Official Documentation
- [Soroban Docs](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [Soroban SDK API](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Stellar CLI Reference](https://developers.stellar.org/docs/tools/stellar-cli)

### Example Repositories
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Soroban Token Example](https://github.com/stellar/soroban-examples/tree/main/token)

### Learning Resources
- [Stellar Quest](https://quest.stellar.org/)
- [Soroban Learn](https://soroban.stellar.org/docs)

### Tools
- [Stellar Laboratory](https://laboratory.stellar.org/)
- [Stellar Expert](https://stellar.expert/)
- [Soroban RPC](https://soroban-rpc.stellar.org/)

---

## 🛠️ Useful Commands

```bash
# Create new contract
stellar contract init my-contract --name my_contract

# Build specific contract
cd contracts/01-counter && stellar contract build

# Build all contracts
stellar contract build

# Run all tests
cargo test

# Run specific test
cargo test --package my-contract test_name

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
  --source <SECRET_KEY> \
  --network testnet

# Invoke contract function
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <SECRET_KEY> \
  --network testnet \
  -- \
  function_name --arg1 value1

# Generate bindings
stellar contract bindings typescript \
  --wasm target/.../contract.wasm \
  --output-dir ./bindings
```

---

## 📝 Notes for Solidity Developers

1. **No Inheritance**: Soroban doesn't have contract inheritance. Use traits and composition.
2. **No Fallback/Receive**: No concept of fallback functions.
3. **No Native Currency in Same Way**: Stellar Lumens (XLM) handled differently than ETH.
4. **Explicit Everything**: More explicit about storage, auth, and state.
5. **Different Testing**: Rust's testing framework, not Hardhat/Foundry.
6. **Compile to WASM**: Target is WebAssembly, not EVM bytecode.

---

**Happy Coding! 🎉**

*Created for Soroban learning journey - Updated: January 2026*

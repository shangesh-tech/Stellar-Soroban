#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

// Storage Key - Using short symbol (9 chars max)
// COUNT = 5 chars, so we use symbol_short!
const COUNT: Symbol = symbol_short!("COUNT");

// TTL Constants (in ledgers, ~5 seconds per ledger)
// Stellar produces ~1 ledger every 5 seconds
// 1 day ≈ 17,280 ledgers (24 * 60 * 60 / 5)
// 1 week ≈ 120,960 ledgers
// 30 days ≈ 518,400 ledgers

const DAY_IN_LEDGERS: u32 = 17_280;
const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;      // Extend by 7 days
const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS;   // Extend when < 1 day left

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    
    /// Increment the counter by 1
    pub fn increment(env: Env) -> u32 {
        // IMPORTANT: Extend TTL on every state-changing operation
        Self::extend_instance_ttl(&env);
        
        let mut count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNT, &count);
        
        return count;
    }

    /// Decrement the counter by 1
    pub fn decrement(env: Env) -> u32 {
        Self::extend_instance_ttl(&env);
        
        let mut count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0);
        
        if count > 0 {
            count -= 1;
        }
        
        env.storage().instance().set(&COUNT, &count);
        return count;
    }

    /// Add a specific value to the counter
    pub fn add(env: Env, value: u32) -> u32 {
        Self::extend_instance_ttl(&env);
        
        let mut count: u32 = env.storage().instance().get(&COUNT).unwrap_or(0);
        count += value;
        env.storage().instance().set(&COUNT, &count);
        
        return count;
    }

    /// Get the current count value (read-only, but still good to extend TTL)
    pub fn get_count(env: Env) -> u32 {
        // Optional: extend TTL even on reads to keep data alive
        Self::extend_instance_ttl(&env);
        
        return env.storage().instance().get(&COUNT).unwrap_or(0);
    }

    /// Reset the counter to 0
    pub fn reset(env: Env) {
        Self::extend_instance_ttl(&env);
        
        env.storage().instance().set(&COUNT, &0u32);
    }
    
    // ============================================
    // INTERNAL HELPER FUNCTION FOR TTL MANAGEMENT
    // ============================================
    
    /// Extend the TTL of the contract instance storage
    /// This keeps the contract and its data alive on the blockchain
    fn extend_instance_ttl(env: &Env) {
        // extend_ttl(threshold, extend_to)
        // - threshold: If TTL < threshold, extend it
        // - extend_to: Extend TTL to this many ledgers from now
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}

mod test;

/*
╔══════════════════════════════════════════════════════════════════════════════╗
║                           TTL DRY RUN EXAMPLE                                ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  SCENARIO: Without TTL Extension                                            ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║                                                                              ║
║  Ledger #1000:     Deploy contract, COUNT = 0                                ║
║                    Default TTL = ~30 days (518,400 ledgers)                  ║
║                    Expires at: Ledger #519,400                               ║
║                                                                              ║
║  Ledger #100,000:  increment() → COUNT = 1                                   ║
║                    TTL NOT extended! Still expires at #519,400               ║
║                                                                              ║
║  Ledger #519,400:  ⚠️ DATA EXPIRED! COUNT is GONE!                           ║
║                    Contract still exists but storage is wiped                ║
║                                                                              ║
║  Ledger #519,401:  get_count() → returns 0 (not 1!)                          ║
║                    Data was lost forever! 💀                                 ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  SCENARIO: With TTL Extension (Our Updated Code)                             ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║                                                                              ║
║  Ledger #1000:     Deploy contract, COUNT = 0                                ║
║                    TTL = 7 days (120,960 ledgers)                            ║
║                    Expires at: Ledger #121,960                               ║
║                                                                              ║
║  Ledger #100,000:  increment() → COUNT = 1                                   ║
║                    extend_ttl(17,280, 120,960) called                        ║
║                    Current TTL = 21,960 ledgers (< 17,280? NO)               ║
║                    TTL NOT extended (still healthy)                          ║
║                    Still expires at: #121,960                                ║
║                                                                              ║
║  Ledger #118,000:  increment() → COUNT = 2                                   ║
║                    extend_ttl(17,280, 120,960) called                        ║
║                    Current TTL = 3,960 ledgers (< 17,280? YES! ✅)           ║
║                    TTL EXTENDED! New expiry: #118,000 + 120,960 = #238,960   ║
║                                                                              ║
║  Ledger #238,960:  If no activity, data expires                              ║
║                    But if anyone calls increment(), TTL extends again!       ║
║                                                                              ║
║  Result: As long as the contract is used regularly, data never expires! ✅   ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

╔══════════════════════════════════════════════════════════════════════════════╗
║                        STORAGE TYPES & TTL SUMMARY                           ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  1. TEMPORARY Storage                                                        ║
║     └─ Use: Session data, temporary calculations                             ║
║     └─ TTL: Very short (~24 hours)                                           ║
║     └─ Cost: Cheapest (no rent)                                              ║
║     └─ Example: Voting session, game state                                   ║
║                                                                              ║
║  2. PERSISTENT Storage                                                       ║
║     └─ Use: User balances, NFT ownership, long-term records                  ║
║     └─ TTL: Configurable, requires rent payment                              ║
║     └─ Cost: More expensive (pay-per-ledger)                                 ║
║     └─ Example: Token balances, user profiles                                ║
║                                                                              ║
║  3. INSTANCE Storage (What we're using)                                      ║
║     └─ Use: Contract-wide settings, global state                             ║
║     └─ TTL: Tied to contract instance                                        ║
║     └─ Cost: Medium (bundled with contract)                                  ║
║     └─ Example: Admin address, contract config, counters                     ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
*/

#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");
static mut x: u32 = 0;
    
#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }

    pub fn make_count_by_1(env: Env) -> u32 {
            x += 1;
            return x;
    }

    pub fn reset(env: Env) {
        env.storage().instance().remove(&COUNTER);
            x = 0;
    }

    pub fn get_count(env: Env) -> u32 {
            return x;
    }

    pub fn make_count_by_value(env: Env, val: u32) -> u32 {
            x += val;
            return x;
    }
}

mod test;
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, 
    symbol_short, Address, Env, String, Symbol,
};

// ═══════════════════════════════════════════════════════════════════════════════
//                              STORAGE KEYS
// ═══════════════════════════════════════════════════════════════════════════════

const TOTAL_YES: Symbol = symbol_short!("yes");      // Total YES votes
const TOTAL_NO: Symbol = symbol_short!("no");        // Total NO votes
const TOTAL_VOTES: Symbol = symbol_short!("total");  // Total votes overall

// ═══════════════════════════════════════════════════════════════════════════════
//                              TTL CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════
// TTL is measured in LEDGERS (1 ledger ≈ 5-6 seconds)
// ~17,280 ledgers ≈ 1 day
// ~120,960 ledgers ≈ 1 week
// ~518,400 ledgers ≈ 1 month

const DAY_IN_LEDGERS: u32 = 17_280;
const WEEK_IN_LEDGERS: u32 = 120_960;
const MONTH_IN_LEDGERS: u32 = 518_400;
const YEAR_IN_LEDGERS: u32 = 6_307_200;

// TTL for voter records (how long to keep individual voter data)
const VOTER_RECORD_TTL: u32 = YEAR_IN_LEDGERS;           // 1 year
const VOTER_RECORD_TTL_THRESHOLD: u32 = MONTH_IN_LEDGERS; // Extend if less than 1 month left

// TTL for instance data (global totals)
const INSTANCE_TTL: u32 = YEAR_IN_LEDGERS;               // 1 year
const INSTANCE_TTL_THRESHOLD: u32 = MONTH_IN_LEDGERS;    // Extend if less than 1 month left

// ═══════════════════════════════════════════════════════════════════════════════
//                         VOTER RECORD
// ═══════════════════════════════════════════════════════════════════════════════

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoterRecord {
    pub voter: Address,    // Real Stellar address
    pub choice: String,    // "yes" or "no"
    pub votes: u64,        // Number of votes (1-5)
    pub timestamp: u64,    // When they voted
}

// ═══════════════════════════════════════════════════════════════════════════════
//                              MAIN CONTRACT
// ═══════════════════════════════════════════════════════════════════════════════

#[contract]
pub struct VoteContract;

#[contractimpl]
impl VoteContract {
    
    // ─────────────────────────────────────────────────────────────────────────
    //                     MAIN VOTING FUNCTION
    // ─────────────────────────────────────────────────────────────────────────
    pub fn vote(env: Env, voter: Address, choice: String, vote_count: u64) {
        // ═══════════════════════════════════════════════════════════════════
        //              AUTHENTICATION
        // ═══════════════════════════════════════════════════════════════════
        voter.require_auth();
        
        // ═══════════════════════════════════════════════════════════════════
        //                     VALIDATION CHECKS
        // ═══════════════════════════════════════════════════════════════════
        assert!(vote_count >= 1, "Must cast at least 1 vote");
        assert!(vote_count <= 5, "Maximum 5 votes allowed");
        
        let yes = String::from_str(&env, "yes");
        let no = String::from_str(&env, "no");
        assert!(choice == yes || choice == no, "Choice must be 'yes' or 'no'");
        
        let existing: Option<VoterRecord> = env.storage().persistent().get(&voter);
        assert!(existing.is_none(), "You have already voted!");
        
        // ═══════════════════════════════════════════════════════════════════
        //                     RECORD THE VOTE
        // ═══════════════════════════════════════════════════════════════════
        let record = VoterRecord {
            voter: voter.clone(),
            choice: choice.clone(),
            votes: vote_count,
            timestamp: env.ledger().timestamp(),
        };
        
        // Save voter record
        env.storage().persistent().set(&voter, &record);
        
        // ═══════════════════════════════════════════════════════════════════
        //                     SET TTL FOR VOTER RECORD
        // ═══════════════════════════════════════════════════════════════════
        // extend_ttl(threshold, extend_to)
        // If TTL < threshold, extend to extend_to
        env.storage().persistent().extend_ttl(
            &voter, 
            VOTER_RECORD_TTL_THRESHOLD,  // If less than 1 month left
            VOTER_RECORD_TTL             // Extend to 1 year
        );
        
        // ═══════════════════════════════════════════════════════════════════
        //                     UPDATE TOTALS
        // ═══════════════════════════════════════════════════════════════════
        if choice == yes {
            let current_yes: u64 = env.storage().instance().get(&TOTAL_YES).unwrap_or(0);
            env.storage().instance().set(&TOTAL_YES, &(current_yes + vote_count));
        } else {
            let current_no: u64 = env.storage().instance().get(&TOTAL_NO).unwrap_or(0);
            env.storage().instance().set(&TOTAL_NO, &(current_no + vote_count));
        }
        
        let total: u64 = env.storage().instance().get(&TOTAL_VOTES).unwrap_or(0);
        env.storage().instance().set(&TOTAL_VOTES, &(total + vote_count));
        
        // ═══════════════════════════════════════════════════════════════════
        //                     EXTEND INSTANCE TTL
        // ═══════════════════════════════════════════════════════════════════
        // Every time someone votes, extend the contract's life
        env.storage().instance().extend_ttl(
            INSTANCE_TTL_THRESHOLD,  // If less than 1 month left
            INSTANCE_TTL             // Extend to 1 year
        );
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    //                     VIEW FUNCTIONS
    // ─────────────────────────────────────────────────────────────────────────
    
    pub fn get_yes_votes(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_YES).unwrap_or(0)
    }
    
    pub fn get_no_votes(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_NO).unwrap_or(0)
    }
    
    pub fn get_total_votes(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_VOTES).unwrap_or(0)
    }
    
    pub fn get_voter(env: Env, voter: Address) -> Option<VoterRecord> {
        env.storage().persistent().get(&voter)
    }
    
    pub fn has_voted(env: Env, voter: Address) -> bool {
        env.storage().persistent().has(&voter)
    }
    
    pub fn get_results(env: Env) -> (u64, u64, u64) {
        let yes = Self::get_yes_votes(env.clone());
        let no = Self::get_no_votes(env.clone());
        let total = Self::get_total_votes(env);
        (yes, no, total)
    }
    
    pub fn get_winner(env: Env) -> String {
        let yes = Self::get_yes_votes(env.clone());
        let no = Self::get_no_votes(env.clone());
        
        if yes > no {
            String::from_str(&env, "yes")
        } else if no > yes {
            String::from_str(&env, "no")
        } else {
            String::from_str(&env, "tie")
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    //                     TTL MANAGEMENT FUNCTIONS
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Extend the contract instance TTL (anyone can call this)
    /// Useful to keep the contract alive if no one is voting
    pub fn extend_contract_ttl(env: Env) {
        env.storage().instance().extend_ttl(
            INSTANCE_TTL_THRESHOLD,
            INSTANCE_TTL
        );
    }
    
    /// Extend a specific voter's record TTL
    /// The voter can call this to keep their vote record alive
    pub fn extend_voter_ttl(env: Env, voter: Address) {
        // Only the voter can extend their own record
        voter.require_auth();
        
        assert!(
            env.storage().persistent().has(&voter),
            "Voter record not found"
        );
        
        env.storage().persistent().extend_ttl(
            &voter,
            VOTER_RECORD_TTL_THRESHOLD,
            VOTER_RECORD_TTL
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//                                  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_vote_yes() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        
        client.vote(&alice, &yes, &3);
        
        assert_eq!(client.get_yes_votes(), 3);
        assert_eq!(client.get_no_votes(), 0);
        assert_eq!(client.get_total_votes(), 3);
        assert_eq!(client.has_voted(&alice), true);
    }
    
    #[test]
    fn test_vote_no() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let bob = Address::generate(&env);
        let no = String::from_str(&env, "no");
        
        client.vote(&bob, &no, &5);
        
        assert_eq!(client.get_yes_votes(), 0);
        assert_eq!(client.get_no_votes(), 5);
        assert_eq!(client.get_total_votes(), 5);
    }
    
    #[test]
    fn test_multiple_voters() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let charlie = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        let no = String::from_str(&env, "no");
        
        client.vote(&alice, &yes, &3);
        client.vote(&bob, &no, &5);
        client.vote(&charlie, &yes, &2);
        
        assert_eq!(client.get_yes_votes(), 5);
        assert_eq!(client.get_no_votes(), 5);
        assert_eq!(client.get_total_votes(), 10);
        assert_eq!(client.get_winner(), String::from_str(&env, "tie"));
    }
    
    #[test]
    fn test_get_voter_record() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        
        client.vote(&alice, &yes, &4);
        
        let record = client.get_voter(&alice);
        assert!(record.is_some());
        
        let record = record.unwrap();
        assert_eq!(record.voter, alice);
        assert_eq!(record.choice, yes);
        assert_eq!(record.votes, 4);
    }
    
    #[test]
    #[should_panic(expected = "You have already voted")]
    fn test_cannot_vote_twice() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        let no = String::from_str(&env, "no");
        
        client.vote(&alice, &yes, &3);
        client.vote(&alice, &no, &2);
    }
    
    #[test]
    #[should_panic(expected = "Maximum 5 votes")]
    fn test_max_votes() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        
        client.vote(&alice, &yes, &10);
    }
    
    #[test]
    #[should_panic(expected = "Must cast at least 1")]
    fn test_min_votes() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        
        client.vote(&alice, &yes, &0);
    }
    
    #[test]
    #[should_panic(expected = "Choice must be")]
    fn test_invalid_choice() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        let alice = Address::generate(&env);
        let invalid = String::from_str(&env, "maybe");
        
        client.vote(&alice, &invalid, &3);
    }
    
    #[test]
    fn test_extend_ttl() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register(VoteContract, ());
        let client = VoteContractClient::new(&env, &contract_id);
        
        // Extend contract TTL (should not panic)
        client.extend_contract_ttl();
        
        // Vote first, then extend voter TTL
        let alice = Address::generate(&env);
        let yes = String::from_str(&env, "yes");
        client.vote(&alice, &yes, &3);
        
        // Extend Alice's voter record TTL
        client.extend_voter_ttl(&alice);
    }
}

/*
╔══════════════════════════════════════════════════════════════════════════════╗
║                         TTL (TIME TO LIVE) GUIDE                             ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  WHAT IS TTL?                                                                ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • TTL = How many ledgers until data expires                                 ║
║  • Soroban data EXPIRES (unlike Ethereum where data lives forever)           ║
║  • This keeps the blockchain efficient                                       ║
║                                                                              ║
║  TIME CONVERSIONS:                                                           ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • 1 ledger ≈ 5-6 seconds                                                    ║
║  • 17,280 ledgers ≈ 1 day                                                    ║
║  • 120,960 ledgers ≈ 1 week                                                  ║
║  • 518,400 ledgers ≈ 1 month                                                 ║
║  • 6,307,200 ledgers ≈ 1 year                                                ║
║                                                                              ║
║  extend_ttl(threshold, extend_to):                                           ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • threshold: Check if current TTL < this value                              ║
║  • extend_to: If so, extend TTL to this value                                ║
║                                                                              ║
║  EXAMPLE:                                                                    ║
║  ┌─────────────────────────────────────────────────────────────────────┐    ║
║  │  extend_ttl(MONTH, YEAR)                                             │    ║
║  │                                                                      │    ║
║  │  "If less than 1 month until expiry, extend to 1 year"               │    ║
║  │                                                                      │    ║
║  │  Current TTL: 2 months → Do nothing (still > 1 month)                │    ║
║  │  Current TTL: 15 days  → Extend to 1 year (< 1 month!)               │    ║
║  └─────────────────────────────────────────────────────────────────────┘    ║
║                                                                              ║
║  STORAGE TYPE TTL Behavior:                                                  ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  INSTANCE:   All entries share same TTL (extends together)                   ║
║  PERSISTENT: Each entry has independent TTL (extend individually)            ║
║  TEMPORARY:  Short TTL, cannot extend much                                   ║
║                                                                              ║
║  WHY DID I ADD extend_contract_ttl() and extend_voter_ttl()?                 ║
║  ─────────────────────────────────────────────────────────────────────────── ║
║  • extend_contract_ttl: Anyone can keep contract alive                       ║
║  • extend_voter_ttl: Voter can keep their vote record alive                  ║
║  • These are "maintenance" functions to prevent data expiry                  ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
*/
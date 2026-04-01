#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, Address, Map, panic_with_error
};

#[derive(Clone)]
#[contracttype]
pub enum Error {
    AlreadyClaimed = 1,
    ContractCallFailed = 2,
}

#[contracttype]
#[derive(Clone)]
pub struct Reward {
    pub voter: Address,
    pub amount: u128,
    pub claimed: bool,
}

const REWARD_AMOUNT: u128 = 1_000_000; // Base units
const VOTERS_KEY: Symbol = Symbol::short("VOTERS");

#[contract]
pub struct RewardContract;

#[contractimpl]
impl RewardContract {
    /// Register voter after successful vote on Poll
    pub fn register_voter(env: Env, voter: Address) {
        let mut voters: Map<Address, Reward> = env.storage()
            .instance()
            .get(&VOTERS_KEY)
            .unwrap_or(Map::new(&env));

        // Check if already claimed
        if let Some(mut existing) = voters.get(voter.clone()) {
            panic_with_error!(&env, Error::AlreadyClaimed);
        }

        // Create new reward entry
        let reward = Reward {
            voter: voter.clone(),
            amount: REWARD_AMOUNT,
            claimed: false,
        };

        voters.set(voter, reward);
        env.storage().instance().set(&VOTERS_KEY, &voters);
    }

    /// Claim reward for voter
    pub fn claim_reward(env: Env, voter: Address) -> u128 {
        let mut voters: Map<Address, Reward> = env.storage()
            .instance()
            .get(&VOTERS_KEY)
            .expect("No voters registered");

        let mut reward = voters.get(voter.clone())
            .expect("Voter not found");

        if reward.claimed {
            panic_with_error!(&env, Error::AlreadyClaimed);
        }

        reward.claimed = true;
        voters.set(voter, reward.clone());
        env.storage().instance().set(&VOTERS_KEY, &voters);

        reward.amount
    }

    /// Get reward status for voter
    pub fn get_reward_status(env: Env, voter: Address) -> (bool, u128) {
        let voters: Map<Address, Reward> = env.storage()
            .instance()
            .get(&VOTERS_KEY)
            .unwrap_or(Map::new(&env));

        match voters.get(voter) {
            Some(reward) => (reward.claimed, reward.amount),
            None => (false, 0),
        }
    }

    /// Get total voters count
    pub fn get_total_voters(env: Env) -> u32 {
        let voters: Map<Address, Reward> = env.storage()
            .instance()
            .get(&VOTERS_KEY)
            .unwrap_or(Map::new(&env));

        voters.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_register_voter() {
        let env = Env::default();
        let contract_id = env.register(RewardContract, ());
        let client = RewardContractClient::new(&env, &contract_id);
        
        let voter = Address::random(&env);
        client.register_voter(&voter);
        
        let (claimed, amount) = client.get_reward_status(&voter);
        assert_eq!(claimed, false);
        assert_eq!(amount, REWARD_AMOUNT);
    }

    #[test]
    fn test_claim_reward() {
        let env = Env::default();
        let contract_id = env.register(RewardContract, ());
        let client = RewardContractClient::new(&env, &contract_id);
        
        let voter = Address::random(&env);
        client.register_voter(&voter);
        
        let claimed_amount = client.claim_reward(&voter);
        assert_eq!(claimed_amount, REWARD_AMOUNT);
        
        let (claimed, _) = client.get_reward_status(&voter);
        assert_eq!(claimed, true);
    }

    #[test]
    fn test_total_voters() {
        let env = Env::default();
        let contract_id = env.register(RewardContract, ());
        let client = RewardContractClient::new(&env, &contract_id);
        
        let voter1 = Address::random(&env);
        let voter2 = Address::random(&env);
        
        client.register_voter(&voter1);
        client.register_voter(&voter2);
        
        let total = client.get_total_voters();
        assert_eq!(total, 2);
    }
}

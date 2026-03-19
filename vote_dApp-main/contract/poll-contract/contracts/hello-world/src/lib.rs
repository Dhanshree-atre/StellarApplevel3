//  #![no_std]

//  use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Map};

//  #[contracttype]
//  pub struct Poll {
//      pub question: Symbol,
//      pub options: Vec<Symbol>,
//      pub votes: Map<Symbol, u32>,
//  }

//  #[contract]
//  pub struct PollContract;

//  #[contractimpl]
//  impl PollContract {

//      pub fn create_poll(env: Env, question: Symbol, options: Vec<Symbol>) {
//          let mut votes = Map::new(&env);
//          for option in options.iter() {
//              votes.set(option.clone(), 0);
//          }

//          let poll = Poll {
//              question,
//              options,
//              votes,
//          };

//          env.storage().instance().set(&Symbol::new(&env, "POLL"), &poll);
//      }

//      pub fn vote(env: Env, option: Symbol) {
//          let mut poll: Poll = env.storage().instance()
//              .get(&Symbol::new(&env, "POLL"))
//              .unwrap();

//          let count = poll.votes.get(option.clone()).unwrap_or(0);
//          poll.votes.set(option, count + 1);

//          env.storage().instance().set(&Symbol::new(&env, "POLL"), &poll);
//      }

//      pub fn get_results(env: Env) -> Map<Symbol, u32> {
//          let poll: Poll = env.storage().instance()
//             .get(&Symbol::new(&env, "POLL"))
//              .unwrap();

//          poll.votes
//      }
//  }













#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, Vec, Map
};

const POLL_KEY: Symbol = Symbol::short("POLL");

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub question: Symbol,
    pub options: Vec<Symbol>,
    pub votes: Map<Symbol, u32>,
}

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {

    pub fn create_poll(env: Env, question: Symbol, options: Vec<Symbol>) {
        if env.storage().instance().has(&POLL_KEY) {
            panic!("Poll already exists");
        }

        let mut votes = Map::new(&env);

        for option in options.iter() {
            votes.set(option.clone(), 0);
        }

        let poll = Poll {
            question,
            options,
            votes,
        };

        env.storage().instance().set(&POLL_KEY, &poll);
    }

    pub fn vote(env: Env, option: Symbol) {
        let mut poll: Poll = env.storage()
            .instance()
            .get(&POLL_KEY)
            .expect("Poll not found");

        if !poll.options.contains(&option) {
            panic!("Invalid option");
        }

        let count = poll.votes.get(option.clone()).unwrap_or(0);
        poll.votes.set(option.clone(), count + 1);

        env.storage().instance().set(&POLL_KEY, &poll);
    }

    pub fn get_poll(env: Env) -> Poll {
        env.storage()
            .instance()
            .get(&POLL_KEY)
            .expect("Poll not found")
    }

    pub fn get_results(env: Env) -> Map<Symbol, u32> {
        let poll: Poll = env.storage()
            .instance()
            .get(&POLL_KEY)
            .expect("Poll not found");

        poll.votes
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Symbol, Vec};

    #[test]
    fn test_create_poll() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PollContract);
        let client = PollContractClient::new(&env, &contract_id);

        let options = Vec::from_array(&env, [
            Symbol::short("Yes"),
            Symbol::short("No")
        ]);

        client.create_poll(&Symbol::short("Q1"), &options);

        let poll = client.get_poll();
        assert_eq!(poll.options.len(), 2);
    }

    #[test]
    fn test_vote() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PollContract);
        let client = PollContractClient::new(&env, &contract_id);

        let options = Vec::from_array(&env, [
            Symbol::short("Yes"),
            Symbol::short("No")
        ]);

        client.create_poll(&Symbol::short("Q1"), &options);
        client.vote(&Symbol::short("Yes"));

        let results = client.get_results();
        assert_eq!(results.get(Symbol::short("Yes")).unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_vote() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PollContract);
        let client = PollContractClient::new(&env, &contract_id);

        let options = Vec::from_array(&env, [
            Symbol::short("Yes"),
            Symbol::short("No")
        ]);

        client.create_poll(&Symbol::short("Q1"), &options);
        client.vote(&Symbol::short("Maybe"));
    }
}
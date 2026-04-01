#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, Symbol};

#[test]
fn test_create_poll() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = PollContractClient::new(&env, &contract_id);

    let question = Symbol::new(&env, "blockchain");
    let options = vec![
        &env,
        Symbol::new(&env, "ethereum"),
        Symbol::new(&env, "solana"),
        Symbol::new(&env, "stellar"),
    ];

    client.create_poll(&question, &options);
    
    let poll = client.get_poll();
    assert_eq!(poll.question, question);
    assert_eq!(poll.options.len(), 3);
}

#[test]
fn test_vote_increments() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = PollContractClient::new(&env, &contract_id);

    let question = Symbol::new(&env, "blockchain");
    let options = vec![
        &env,
        Symbol::new(&env, "ethereum"),
        Symbol::new(&env, "solana"),
        Symbol::new(&env, "stellar"),
    ];

    client.create_poll(&question, &options);

    let ethereum = Symbol::new(&env, "ethereum");
    client.vote(&ethereum);

    let results = client.get_results();
    let votes = results.get(ethereum).unwrap();
    assert_eq!(votes, 1);
}

#[test]
fn test_get_results_correct() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = PollContractClient::new(&env, &contract_id);

    let question = Symbol::new(&env, "best_blockchain");
    let options = vec![
        &env,
        Symbol::new(&env, "stellar"),
        Symbol::new(&env, "ethereum"),
    ];

    client.create_poll(&question, &options);

    let stellar = Symbol::new(&env, "stellar");
    client.vote(&stellar);
    client.vote(&stellar);

    let results = client.get_results();
    assert_eq!(results.get(stellar).unwrap(), 2);
}

#[test]
fn test_multiple_votes() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = PollContractClient::new(&env, &contract_id);

    let question = Symbol::new(&env, "vote");
    let options = vec![
        &env,
        Symbol::new(&env, "yes"),
        Symbol::new(&env, "no"),
    ];

    client.create_poll(&question, &options);

    let yes = Symbol::new(&env, "yes");
    let no = Symbol::new(&env, "no");

    client.vote(&yes);
    client.vote(&yes);
    client.vote(&no);

    let results = client.get_results();
    assert_eq!(results.get(yes).unwrap(), 2);
    assert_eq!(results.get(no).unwrap(), 1);
}

#[test]
fn test_total_votes() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = PollContractClient::new(&env, &contract_id);

    let question = Symbol::new(&env, "payment_choice");
    let options = vec![
        &env,
        Symbol::new(&env, "xlm"),
        Symbol::new(&env, "stablecoin"),
        Symbol::new(&env, "other"),
    ];

    client.create_poll(&question, &options);

    let xlm = Symbol::new(&env, "xlm");
    let stablecoin = Symbol::new(&env, "stablecoin");
    let other = Symbol::new(&env, "other");

    client.vote(&xlm);
    client.vote(&xlm);
    client.vote(&xlm);
    client.vote(&stablecoin);
    client.vote(&stablecoin);
    client.vote(&other);

    let results = client.get_results();
    let total = results.get(xlm).unwrap() + results.get(stablecoin).unwrap() + results.get(other).unwrap();
    assert_eq!(total, 6);
}

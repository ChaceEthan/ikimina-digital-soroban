#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct IkiminaContract;

#[contractimpl]
impl IkiminaContract {
    pub fn hello(env: Env) -> u32 {
        env.ledger().sequence()
    }
}

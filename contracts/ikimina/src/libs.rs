#![no_std]

use soroban_sdk::{contractimpl, Env};

pub struct IkiminaContract;

#[contractimpl]
impl IkiminaContract {
    // Example function
    pub fn hello(env: Env) -> String {
        "Hello Ikimina Digital!".to_string()
    }
}

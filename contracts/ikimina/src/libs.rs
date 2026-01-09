#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol,
};

// Storage keys
#[contracttype]
pub enum DataKey {
    Group(u64),
    GroupCount,
    GroupMembers(u64),
}

// Member structure
#[contracttype]
pub struct Member {
    pub address: Address,
    pub has_received_payout: bool,
}

// IkiminaGroup structure
#[contracttype]
pub struct IkiminaGroup {
    pub group_id: u64,
    pub owner: Address,
    pub name: Symbol,
    pub min_members: u32,
    pub max_members: u32,
    pub contribution_amount: i128,
    pub total_rounds: u32,
    pub current_round: u32,
    pub is_active: bool,
}

// Contract definition
#[contract]
pub struct IkiminaContract;

// Contract implementation
#[contractimpl]
impl IkiminaContract {
    pub fn init(env: Env) {
        let storage = env.storage().instance();
        if storage.has(&DataKey::GroupCount) {
            panic!("Contract already initialized");
        }
        storage.set(&DataKey::GroupCount, &0u64);
    }

    pub fn get_group_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::GroupCount).unwrap_or(0u64)
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct WithdrawalContract;

#[contractimpl]
impl WithdrawalContract {
    pub fn hello(env: Env) -> soroban_sdk::Symbol {
        soroban_sdk::Symbol::new(&env, "withdrawal")
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct DonationContract;

#[contractimpl]
impl DonationContract {
    pub fn hello(env: Env) -> soroban_sdk::Symbol {
        soroban_sdk::Symbol::new(&env, "donation")
    }
}

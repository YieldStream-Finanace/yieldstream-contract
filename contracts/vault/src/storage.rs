use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Asset,
    Stream(Address),
    TotalAllocated,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StreamState {
    pub recipient: Address,
    pub rate_per_second: i128,
    pub start_time: u64,
    pub stop_time: u64,
    pub claimed_amount: i128,
}
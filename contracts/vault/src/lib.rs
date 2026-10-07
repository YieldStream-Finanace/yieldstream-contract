#![no_std]

mod errors;
mod events;
mod storage;

#[cfg(test)]
mod test;

use errors::ContractError;
use events::{emit_stream_canceled, emit_stream_created, emit_yield_withdrawn};
use storage::{DataKey, StreamState};
use soroban_sdk::{contract, contractimpl, token, Address, Env};

pub trait YieldStreamVaultTrait {
    fn initialize(e: Env, admin: Address, asset: Address) -> Result<(), ContractError>;
    fn create_stream(
        e: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        start_time: u64,
        stop_time: u64,
    ) -> Result<(), ContractError>;
    fn withdraw(e: Env, recipient: Address) -> Result<i128, ContractError>;
    fn get_claimable(e: Env, recipient: Address) -> i128;
    fn cancel_stream(e: Env, sender: Address, recipient: Address) -> Result<i128, ContractError>;
}

#[contract]
pub struct YieldStreamVault;

#[contractimpl]
impl YieldStreamVaultTrait for YieldStreamVault {
    fn initialize(e: Env, admin: Address, asset: Address) -> Result<(), ContractError> {
        if e.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::Asset, &asset);
        Ok(())
    }

    fn create_stream(
        e: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        start_time: u64,
        stop_time: u64,
    ) -> Result<(), ContractError> {
        sender.require_auth();

        if amount <= 0 {
            return Err(ContractError::ZeroAmount);
        }
        if stop_time <= start_time {
            return Err(ContractError::InvalidTimeRange);
        }
        if e.storage().persistent().has(&DataKey::Stream(recipient.clone())) {
            return Err(ContractError::StreamAlreadyExists);
        }

        let duration = stop_time - start_time;
        let rate_per_second = amount / (duration as i128);

        let asset: Address = e.storage().instance().get(&DataKey::Asset).unwrap();
        let client = token::Client::new(&e, &asset);
        client.transfer(&sender, &e.current_contract_address(), &amount);

        let stream = StreamState {
            recipient: recipient.clone(),
            rate_per_second,
            start_time,
            stop_time,
            claimed_amount: 0,
        };

        e.storage().persistent().set(&DataKey::Stream(recipient.clone()), &stream);
        emit_stream_created(&e, sender, recipient, amount);

        Ok(())
    }

    fn withdraw(e: Env, recipient: Address) -> Result<i128, ContractError> {
        recipient.require_auth();

        let claimable = Self::get_claimable(e.clone(), recipient.clone());
        if claimable <= 0 {
            return Err(ContractError::NothingToClaim);
        }

        let key = DataKey::Stream(recipient.clone());
        let mut stream: StreamState = e
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::StreamNotFound)?;

        stream.claimed_amount += claimable;
        e.storage().persistent().set(&key, &stream);

        let asset: Address = e.storage().instance().get(&DataKey::Asset).unwrap();
        let client = token::Client::new(&e, &asset);
        client.transfer(&e.current_contract_address(), &recipient, &claimable);

        emit_yield_withdrawn(&e, recipient, claimable);

        Ok(claimable)
    }

    fn get_claimable(e: Env, recipient: Address) -> i128 {
        let key = DataKey::Stream(recipient);
        let stream: Option<StreamState> = e.storage().persistent().get(&key);

        if let Some(s) = stream {
            let now = e.ledger().timestamp();
            if now <= s.start_time {
                return 0;
            }
            let elapsed = if now >= s.stop_time {
                s.stop_time - s.start_time
            } else {
                now - s.start_time
            };

            let total_accrued = s.rate_per_second * (elapsed as i128);
            total_accrued - s.claimed_amount
        } else {
            0
        }
    }

    fn cancel_stream(e: Env, sender: Address, recipient: Address) -> Result<i128, ContractError> {
        sender.require_auth();

        let key = DataKey::Stream(recipient.clone());
        let stream: StreamState = e
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ContractError::StreamNotFound)?;

        let claimable = Self::get_claimable(e.clone(), recipient.clone());
        let duration = stream.stop_time - stream.start_time;
        let total_amount = stream.rate_per_second * (duration as i128);
        let unearned = total_amount - (stream.claimed_amount + claimable);

        e.storage().persistent().remove(&key);

        let asset: Address = e.storage().instance().get(&DataKey::Asset).unwrap();
        let client = token::Client::new(&e, &asset);

        if claimable > 0 {
            client.transfer(&e.current_contract_address(), &recipient, &claimable);
        }
        if unearned > 0 {
            client.transfer(&e.current_contract_address(), &sender, &unearned);
        }

        emit_stream_canceled(&e, sender, recipient, unearned);

        Ok(unearned)
    }
}
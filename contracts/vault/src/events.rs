use soroban_sdk::{symbol_short, Address, Env};

pub fn emit_stream_created(e: &Env, sender: Address, recipient: Address, amount: i128) {
    e.events().publish(
        (symbol_short!("created"), sender),
        (recipient, amount),
    );
}

pub fn emit_yield_withdrawn(e: &Env, recipient: Address, amount: i128) {
    e.events().publish(
        (symbol_short!("withdraw"), recipient),
        amount,
    );
}

pub fn emit_stream_canceled(e: &Env, sender: Address, recipient: Address, refunded: i128) {
    e.events().publish(
        (symbol_short!("canceled"), sender),
        (recipient, refunded),
    );
}
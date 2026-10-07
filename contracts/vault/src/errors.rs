use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidTimeRange = 3,
    ZeroAmount = 4,
    StreamNotFound = 5,
    StreamAlreadyExists = 6,
    Unauthorized = 7,
    NothingToClaim = 8,
}
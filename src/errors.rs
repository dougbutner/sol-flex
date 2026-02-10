use anchor_lang::prelude::*;

#[error_code]
pub enum SolFlexError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid configuration")]
    InvalidConfig,

    #[msg("Account already in blocklist")]
    AlreadyInBlocklist,

    #[msg("Account not found in blocklist")]
    NotInBlocklist,

    #[msg("Blocklist is full")]
    BlocklistFull,

    #[msg("Invalid parameters")]
    InvalidParameters,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Pool already exists")]
    PoolAlreadyExists,

    #[msg("Pool not found")]
    PoolNotFound,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("No reflections to distribute")]
    NoReflectionsToDistribute,

    #[msg("Invalid memo length")]
    InvalidMemoLength,

    #[msg("Account not found")]
    AccountNotFound,
}
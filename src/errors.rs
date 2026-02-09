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
}
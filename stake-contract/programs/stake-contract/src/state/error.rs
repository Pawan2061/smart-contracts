use anchor_lang::prelude::*;

#[error_code]

pub enum StakingError {
    #[msg("Invalid amount of tokens")]
    InvalidAmount,

    #[msg("Overflow of the tokens")]
    Overflow,

    #[msg("Insufficient state of the stake")]
    InsufficientStake,

    #[msg("Underflow of the stake")]
    Underflow,

    #[msg("Insufficient points")]
    InsufficientPoints,
}

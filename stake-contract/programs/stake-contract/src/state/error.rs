use anchor_lang::prelude::*;

#[error_code]

pub enum StakingError {
    #[msg("Invalid amount of tokens")]
    InvalidAmount,

    #[msg("Overflow of the tokens")]
    Overflow,
}

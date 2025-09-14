use anchor_lang::prelude::*;

#[error_code]
pub enum CLMMERROR {
    #[msg("Token A and Token B must be different.")]
    SameTokenMint,

    #[msg("Invalid amount of tokens available")]
    InvalidAmount,
}

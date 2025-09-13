use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Insufficient token balance to initialize escrow.")]
    InsufficientBalance,

    #[msg("The escrow is not active.")]
    EscrowInactive,

    #[msg("Escrow account is not active.")]
    EscrowNotActive,

    #[msg("Unauthorized action attempted.")]
    Unauthorized,

    #[msg("Invalid token mint provided.")]
    InvalidMint,

    #[msg("Expected amount does not match.")]
    AmountMismatch,

    #[msg("Escrow already completed or cancelled.")]
    EscrowClosed,
}

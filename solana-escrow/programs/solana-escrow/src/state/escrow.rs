use anchor_lang::prelude::*;

#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
    pub is_active: bool,
}

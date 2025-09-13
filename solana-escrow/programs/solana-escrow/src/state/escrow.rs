use anchor_lang::prelude::*;

#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub bump: u8,
    pub is_active: bool,
}

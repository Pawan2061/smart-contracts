use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct Pool {
    pub mint_a: Pubkey,         // 32
    pub mint_b: Pubkey,         // 32
    pub vault_a: Pubkey,        // 32
    pub vault_b: Pubkey,        // 32
    pub lp_mint: Pubkey,        // 32
    pub pool_authority: Pubkey, // 32
    pub sqrt_price_x64: u128,   // 16
    pub active_liquidity: u128, // 16
    pub total_lp_issued: u64,   // 8
    pub current_tick: i32,      // 4
    pub bump: u8,               // 1
    pub _padding: [u8; 3],      // 3
}

impl Pool {
    /// Total size of the Pool account (in bytes).
    pub const LEN: usize = 32 * 6   // pubkeys
        + 16 * 2                    // u128s
        + 8                         // u64
        + 4                         // i32
        + 1                         // bump
        + 3; // padding
}

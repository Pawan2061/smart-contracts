use anchor_lang::prelude::*;

#[account]
pub struct Tick {
    pub sqrt_price_x64: u128,
    pub liquidity_net: i128,
    pub index: i32,
    pub bump: u8,
}

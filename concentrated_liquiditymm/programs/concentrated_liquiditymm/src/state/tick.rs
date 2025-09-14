use anchor_lang::prelude::*;

#[account]
pub struct Tick {
    pub liquidity_gross: u128,
    pub liquidity_net: i128,
    pub index: i32,
    pub bump: u8,
}

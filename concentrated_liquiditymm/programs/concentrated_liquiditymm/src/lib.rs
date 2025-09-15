use anchor_lang::prelude::*;

declare_id!("7bo78Lu7uofN94e4Z6aJ1xnni1Brg32qn3Ut6fmcrkj5");
pub mod instructions;
pub mod state;
pub mod utils;
use crate::instructions::*;
#[program]
pub mod concentrated_liquiditymm {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, current_price: u64) -> Result<()> {
        instructions::init_pool(ctx, current_price);
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_a: u64,
        amount_b: u64,

        tick_lower_val: i32,
        tick_upper_val: i32,
    ) -> Result<()> {
        instructions::add_liquidity(ctx, amount_a, amount_b, tick_lower_val, tick_upper_val);
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>, amount_in: u64, zero_for_one: bool) -> Result<()> {
        instructions::exchange(ctx, amount_in, zero_for_one);
        Ok(())
    }

    pub fn withdraw_liquidity(
        ctx: Context<WithdrawLiquidity>,
        lp_amount: u64,
        tick_lower_val: i32,
        tick_upper_val: i32,
    ) -> Result<()> {
        instructions::withdraw_liquidity(ctx, lp_amount, tick_lower_val, tick_upper_val);
        Ok(())
    }
}

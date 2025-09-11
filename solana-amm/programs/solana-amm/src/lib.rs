use anchor_lang::prelude::*;

declare_id!("BvAYVdAqjoiS9zRawpc6dDktotk1NbRiT7QETT2xZVk9");

mod instructions;
mod state;
use instructions::*;
#[program]
pub mod solana_amm {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitPool>) -> Result<()> {
        instructions::initialize_pool::initialize_pool(ctx);
        Ok(())
    }

    pub fn add_liquidity_tokens(
        ctx: Context<AddTokenLiquity>,
        quantity_a: u64,
        quantity_b: u64,
    ) -> Result<()> {
        instructions::add_token_liquidity::add_token_liquidity(ctx, quantity_a, quantity_b);
        Ok(())
    }
    pub fn withdraw(
        ctx: Context<SwapTokenContext>,
        amount_in: u64,
        min_amount_out: u64,
        is_a_to_b: bool,
    ) -> Result<()> {
        instructions::swap_token(ctx, amount_in, min_amount_out, is_a_to_b);
        Ok(())
    }

    pub fn swap(ctx: Context<Initialize>) -> Result<()> {
        msg!("swapping tokens");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

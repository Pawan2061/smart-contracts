use anchor_lang::prelude::*;

declare_id!("BvAYVdAqjoiS9zRawpc6dDktotk1NbRiT7QETT2xZVk9");

mod instructions;
mod state;
use instructions::*;
#[program]
pub mod solana_amm {
    // use crate::instructions::initialize_pool::InitPool;

    use super::*;

    pub fn initialize_pool(ctx: Context<InitPool>) -> Result<()> {
        instructions::initialize_pool::initialize_pool(ctx);
        Ok(())
    }

    pub fn add_liquidity_tokens(ctx: Context<Initialize>) -> Result<()> {
        msg!("working on adding liquidity tokens");
        Ok(())
    }
    pub fn withdraw(ctx: Context<Initialize>) -> Result<()> {
        msg!("working on adding liquidity tokens");
        Ok(())
    }

    pub fn swap(ctx: Context<Initialize>) -> Result<()> {
        msg!("swapping tokens");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

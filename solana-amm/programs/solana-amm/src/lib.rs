use anchor_lang::prelude::*;

declare_id!("BvAYVdAqjoiS9zRawpc6dDktotk1NbRiT7QETT2xZVk9");

#[program]
pub mod solana_amm {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    pub fn initialize_pool(ctx: Context<Initialize>) -> Result<()> {
        msg!("Grettings");
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

use anchor_lang::prelude::*;

declare_id!("7bo78Lu7uofN94e4Z6aJ1xnni1Brg32qn3Ut6fmcrkj5");
pub mod instructions;
pub mod state;
pub mod utils;
#[program]
pub mod concentrated_liquiditymm {
    use super::*;

    pub fn initialize_pool(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn initialize_tick(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn exchange(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn withdraw_liquidity(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

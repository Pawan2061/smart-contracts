use anchor_lang::prelude::*;
declare_id!("FuxeJF3Ff7H6hQ5PuuC7Rp4N9LE2HCaAnmmtBzWmRS2a");
pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::*;

#[program]
pub mod stake_contract {
    use super::*;

    pub fn create_stake_account(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn stake_tokens(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn unstake_tokens(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }

    pub fn claim_reward(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

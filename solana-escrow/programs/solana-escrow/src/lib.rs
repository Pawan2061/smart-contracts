use anchor_lang::prelude::*;

declare_id!("9VhspbbW52zGouawaKVXe4MUz3HAB2xFM7NBhZ8CkLUP");

pub mod instructions;
pub mod state;

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn init_escrow(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring right now");

        Ok(())
    }

    pub fn exchange(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring till now");
        Ok(())
    }

    pub fn cancel_exchange(ctx: Context<Initialize>) -> Result<()> {
        msg!("wokring");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

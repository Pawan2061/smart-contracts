use anchor_lang::prelude::*;

declare_id!("9VhspbbW52zGouawaKVXe4MUz3HAB2xFM7NBhZ8CkLUP");

pub mod instructions;
pub mod state;

use crate::instructions::*;
#[program]
pub mod solana_escrow {
    use super::*;

    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        amount_a: u64,
        mint_b: Pubkey,
        amount_b: u64,
    ) -> Result<()> {
        instructions::init_escrow::init_escrow(ctx, amount_a, mint_b, amount_b)?;

        msg!("Escrow initialized successfully with Token A locked and expecting Token B");

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

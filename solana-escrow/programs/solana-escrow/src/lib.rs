use anchor_lang::prelude::*;

declare_id!("9VhspbbW52zGouawaKVXe4MUz3HAB2xFM7NBhZ8CkLUP");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

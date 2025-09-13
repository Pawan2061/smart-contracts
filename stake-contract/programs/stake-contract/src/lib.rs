use anchor_lang::prelude::*;

declare_id!("FuxeJF3Ff7H6hQ5PuuC7Rp4N9LE2HCaAnmmtBzWmRS2a");

#[program]
pub mod stake_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

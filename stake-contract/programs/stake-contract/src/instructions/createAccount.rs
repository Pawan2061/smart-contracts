use anchor_lang::prelude::*;

pub fn create_stake_account(ctx: Context<CreateAccount>) -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct CreateAccount<'info> {
    pub system_program: Program<'info, System>,
}

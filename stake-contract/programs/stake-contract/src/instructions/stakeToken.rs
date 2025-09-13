use anchor_lang::prelude::*;

pub fn stake_token(ctx: Context<StakeToken>) -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct StakeToken<'info> {
    pub system_program: Program<'info, System>,
}

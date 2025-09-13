use anchor_lang::prelude::*;

pub fn unstake_token(ctx: Context<UnstakeToken>) -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct UnstakeToken<'info> {
    pub system_program: Program<'info, System>,
}

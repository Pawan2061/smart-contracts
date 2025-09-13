use anchor_lang::prelude::*;

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct ClaimReward<'info> {
    pub system_program: Program<'info, System>,
}

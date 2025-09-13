use crate::state::StakeAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn create_stake_account(ctx: Context<CreateAccount>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;

    stake_account.owner = ctx.accounts.user.key();
    stake_account.staked_amount = 0;
    stake_account.total_points = 0;
    stake_account.stake_timestamp = 0;
    stake_account.bump = ctx.bumps.stake_account;

    msg!(
        "Stake account + vault created successfully for user: {}",
        stake_account.owner
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub stake_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"stake_account", user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        init,
        payer = user,
        token::mint = stake_mint,
        token::authority = stake_account,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

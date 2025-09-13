use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::StakeAccount;
use crate::state::StakingError;
pub fn stake_token(ctx: Context<StakeToken>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;
    stake_account.staked_amount = stake_account
        .staked_amount
        .checked_add(amount)
        .ok_or(StakingError::Overflow)?;
    stake_account.stake_timestamp = clock.unix_timestamp;

    msg!("User staked {} tokens!", amount);
    Ok(())
}

#[derive(Accounts)]

pub struct StakeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
    mut,
    seeds = [b"vault", user.key().as_ref()],
    bump,
    token::mint = token_mint,
    token::authority = stake_account
)]
    pub vault: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

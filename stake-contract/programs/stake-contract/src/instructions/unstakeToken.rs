use crate::state::StakingError;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::StakeAccount;
pub fn unstake_token(ctx: Context<UnstakeToken>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let stake_account = &mut ctx.accounts.stake_account;

    require!(
        stake_account.staked_amount >= amount,
        StakingError::InsufficientStake
    );

    let clock = Clock::get()?;
    stake_account.stake_timestamp = clock.unix_timestamp;

    let key = &ctx.accounts.user.key();
    let bump = stake_account.bump;
    let seeds: &[&[u8]] = &[b"vault", key.as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    token::transfer(cpi_ctx, amount)?;

    stake_account.staked_amount = stake_account
        .staked_amount
        .checked_sub(amount)
        .ok_or(StakingError::Underflow)?;

    msg!("User unstaked {} tokens!", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeToken<'info> {
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
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK

    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

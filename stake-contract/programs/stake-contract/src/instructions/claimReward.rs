use crate::state::StakeAccount;
use crate::state::StakingError;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self, MintTo};
pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;

    let clock = Clock::get()?;

    stake_account.stake_timestamp = clock.unix_timestamp;

    let rewards = stake_account.total_points;
    require!(rewards > 0, StakingError::InsufficientPoints);

    let bump = ctx.bumps.mint_authority;
    let seeds: &[&[u8]] = &[b"mint_authority", &[bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.reward_mint.to_account_info(),
        to: ctx.accounts.user_reward_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    token::mint_to(cpi_ctx, rewards)?;

    stake_account.total_points = 0;

    msg!("User claimed {} reward tokens!", rewards);
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_reward_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

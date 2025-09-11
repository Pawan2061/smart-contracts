use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

use crate::state::{AMMPool, SOLAMMERROR};

pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, lp_amount: u64) -> Result<()> {
    msg!("Withdrawing liquidity...");

    let pool = &mut ctx.accounts.amm_pool;

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ),
        lp_amount,
    )?;

    let total_lp_supply = ctx.accounts.lp_mint.supply;
    require!(total_lp_supply > 0, SOLAMMERROR::ZeroAmount);

    let withdraw_a = (lp_amount as u128)
        .checked_mul(ctx.accounts.vault_a.amount as u128)
        .unwrap()
        .checked_div(total_lp_supply as u128)
        .unwrap() as u64;

    let withdraw_b = (lp_amount as u128)
        .checked_mul(ctx.accounts.vault_b.amount as u128)
        .unwrap()
        .checked_div(total_lp_supply as u128)
        .unwrap() as u64;

    let bump = ctx.bumps.authority;

    let token_a_key = ctx.accounts.token_mint_a.key();
    let token_b_key = ctx.accounts.token_mint_b.key();
    let seeds: &[&[u8]] = &[
        b"authority",
        token_a_key.as_ref(),
        token_b_key.as_ref(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_a.to_account_info(),
                to: ctx.accounts.token_account_a.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer_seeds,
        ),
        withdraw_a,
    )?;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_b.to_account_info(),
                to: ctx.accounts.token_account_b.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer_seeds,
        ),
        withdraw_b,
    )?;

    msg!(
        "Withdrew {} of token A and {} of token B",
        withdraw_a,
        withdraw_b
    );

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_mint_a: Account<'info, Mint>,
    pub token_mint_b: Account<'info, Mint>,

    /// CHECK: This PDA is the pool authority. It is derived using seeds and bump,
    /// so its address is deterministic and secure. No further runtime checks needed.
    #[account(
        seeds = [b"authority", token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = signer,
    )]
    pub token_account_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = signer,
    )]
    pub token_account_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_token", token_mint_a.key().as_ref(), token_mint_b.key().as_ref(), b"A"],
        bump,
        token::mint = token_mint_a,
        token::authority = authority,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault_token", token_mint_a.key().as_ref(), token_mint_b.key().as_ref(), b"B"],
        bump,
        token::mint = token_mint_b,
        token::authority = authority,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"lp_mint", token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        bump,
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = signer,
    )]
    pub user_lp_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool", token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        bump
    )]
    pub amm_pool: Account<'info, AMMPool>,

    pub token_program: Program<'info, Token>,
}

use crate::state::error::CLMMERROR;
use crate::state::pool::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::state::tick::Tick;
use crate::utils::*;

pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    amount_a: u64,
    amount_b: u64,
    tick_lower_val: i32,
    tick_upper_val: i32,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;

    require!(amount_a > 0 && amount_b > 0, CLMMERROR::InvalidAmount);
    require!(tick_lower_val < tick_upper_val, CLMMERROR::InvalidTickRange);

    let liquidity_to_add = (amount_a as u128).min(amount_b as u128);
    ctx.accounts.tick_lower.liquidity_net += liquidity_to_add as i128;
    ctx.accounts.tick_lower.liquidity_gross += liquidity_to_add;

    ctx.accounts.tick_upper.liquidity_net -= liquidity_to_add as i128;
    ctx.accounts.tick_upper.liquidity_gross += liquidity_to_add;

    if pool.current_tick >= tick_lower_val && pool.current_tick < tick_upper_val {
        pool.active_liquidity += liquidity_to_add;
    }

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts_a = Transfer {
        from: ctx.accounts.token_account_a.to_account_info(),
        to: ctx.accounts.vault_a.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    token::transfer(
        CpiContext::new(cpi_program.clone(), cpi_accounts_a),
        amount_a,
    )?;

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.token_account_b.to_account_info(),
        to: ctx.accounts.vault_b.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    token::transfer(
        CpiContext::new(cpi_program.clone(), cpi_accounts_b),
        amount_b,
    )?;

    let mint_a = ctx.accounts.mint_a.key();

    let mint_b = ctx.accounts.mint_b.key();
    let seeds: &[&[u8]] = &[b"authority", mint_a.as_ref(), mint_b.as_ref(), &[pool.bump]];
    let signer: &[&[&[u8]]] = &[seeds];

    let lp_to_mint = if pool.total_lp_issued == 0 {
        integer_sqrt(amount_a as u128 * amount_b as u128) as u64
    } else {
        let lp_a = (amount_a as u128 * pool.total_lp_issued as u128
            / ctx.accounts.vault_a.amount as u128) as u64;
        let lp_b = (amount_b as u128 * pool.total_lp_issued as u128
            / ctx.accounts.vault_b.amount as u128) as u64;
        lp_a.min(lp_b)
    };

    let cpi_accounts_mint = MintTo {
        mint: ctx.accounts.lp_mint.to_account_info(),
        to: ctx.accounts.user_lp.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::mint_to(
        CpiContext::new_with_signer(cpi_program, cpi_accounts_mint, signer),
        lp_to_mint,
    )?;

    pool.total_lp_issued += lp_to_mint;

    Ok(())
}

#[derive(Accounts)]
#[instruction(tick_lower_val: i32, tick_upper_val: i32)]

pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(mut, seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub pool: AccountLoader<'info, Pool>,

    #[account(mut)]
    pub vault_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_account_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_account_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"tick", pool.key().as_ref(), &tick_lower_val.to_le_bytes()], bump)]
    pub tick_lower: Account<'info, Tick>,

    #[account(mut, seeds = [b"tick", pool.key().as_ref(), &tick_upper_val.to_le_bytes()], bump)]
    pub tick_upper: Account<'info, Tick>,

    /// CHECK: This PDA is derived using seeds [b"authority", mint_a, mint_b].
    /// We don’t need to deserialize or enforce ownership, just use it as a PDA signer.
    #[account(seeds = [b"authority", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

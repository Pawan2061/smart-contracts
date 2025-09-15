use crate::state::error::CLMMERROR;
use crate::state::pool::Pool;
use crate::state::tick::Tick;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

pub fn withdraw_liquidity(
    ctx: Context<WithdrawLiquidity>,
    lp_amount: u64,
    tick_lower_val: i32,
    tick_upper_val: i32,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;

    require!(lp_amount > 0, CLMMERROR::InvalidAmount);

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts_burn = Burn {
        mint: ctx.accounts.lp_mint.to_account_info(),
        from: ctx.accounts.user_lp.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    token::burn(
        CpiContext::new(cpi_program.clone(), cpi_accounts_burn),
        lp_amount,
    )?;

    let share = lp_amount as u128 * 1_000_000_000u128 / pool.total_lp_issued as u128;

    let amount_a = (ctx.accounts.vault_a.amount as u128 * share / 1_000_000_000u128) as u64;
    let amount_b = (ctx.accounts.vault_b.amount as u128 * share / 1_000_000_000u128) as u64;

    ctx.accounts.tick_lower.liquidity_net -= amount_a.min(amount_b) as i128;
    ctx.accounts.tick_lower.liquidity_gross -= amount_a.min(amount_b) as u128;

    ctx.accounts.tick_upper.liquidity_net += amount_a.min(amount_b) as i128;
    ctx.accounts.tick_upper.liquidity_gross -= amount_a.min(amount_b) as u128;

    if pool.current_tick >= tick_lower_val && pool.current_tick < tick_upper_val {
        pool.active_liquidity -= amount_a.min(amount_b) as u128;
    }

    let a_mint = ctx.accounts.mint_a.key();

    let b_mint = ctx.accounts.mint_b.key();
    let seeds: &[&[u8]] = &[b"authority", a_mint.as_ref(), b_mint.as_ref(), &[pool.bump]];
    let signer: &[&[&[u8]]] = &[seeds];

    let cpi_accounts_a = Transfer {
        from: ctx.accounts.vault_a.to_account_info(),
        to: ctx.accounts.token_account_a.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts_a, signer),
        amount_a,
    )?;

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.vault_b.to_account_info(),
        to: ctx.accounts.token_account_b.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts_b, signer),
        amount_b,
    )?;

    pool.total_lp_issued -= lp_amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(tick_lower_val: i32, tick_upper_val: i32)]
pub struct WithdrawLiquidity<'info> {
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

    #[account(seeds = [b"authority", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::error::CLMMERROR;
use crate::state::pool::Pool;
use crate::state::tick::Tick;

use crate::utils::*;

pub fn exchange(ctx: Context<Exchange>, amount_in: u64, zero_for_one: bool) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;
    require!(amount_in > 0, CLMMERROR::InvalidAmount);

    let mut remaining_amount = amount_in as u128;
    let mut amount_out_accum: u128 = 0;

    let mut current_tick = pool.current_tick;
    let mut sqrt_price_x64 = pool.sqrt_price_x64;
    let mut liquidity = pool.active_liquidity;

    while remaining_amount > 0 {
        let next_tick = if zero_for_one {
            ctx.accounts.tick_lower.index
        } else {
            ctx.accounts.tick_upper.index
        };

        let next_sqrt_price = tick_to_sqrt_price_x64(next_tick)?;

        let (consumed_in, produced_out, new_sqrt_price) = swap_step(
            sqrt_price_x64,
            next_sqrt_price,
            liquidity,
            remaining_amount,
            zero_for_one,
        )?;

        remaining_amount -= consumed_in;
        amount_out_accum += produced_out;
        sqrt_price_x64 = new_sqrt_price;

        if sqrt_price_x64 == next_sqrt_price {
            if zero_for_one {
                liquidity = (liquidity as i128 + ctx.accounts.tick_lower.liquidity_net) as u128;
                current_tick = next_tick;
            } else {
                liquidity = (liquidity as i128 + ctx.accounts.tick_upper.liquidity_net) as u128;
                current_tick = next_tick;
            }
        } else {
            break;
        }
    }

    pool.sqrt_price_x64 = sqrt_price_x64;
    pool.current_tick = current_tick;
    pool.active_liquidity = liquidity;

    let cpi_program = ctx.accounts.token_program.to_account_info();

    if zero_for_one {
        let transfer_in = Transfer {
            from: ctx.accounts.user_token_a.to_account_info(),
            to: ctx.accounts.vault_a.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(CpiContext::new(cpi_program.clone(), transfer_in), amount_in)?;

        let transfer_out = Transfer {
            from: ctx.accounts.vault_b.to_account_info(),
            to: ctx.accounts.user_token_b.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let a_mint: Pubkey = ctx.accounts.mint_a.key();
        let b_mint: Pubkey = ctx.accounts.mint_b.key();

        let seeds: &[&[u8]] = &[b"authority", a_mint.as_ref(), b_mint.as_ref(), &[pool.bump]];
        token::transfer(
            CpiContext::new_with_signer(cpi_program.clone(), transfer_out, &[seeds]),
            amount_out_accum as u64,
        )?;
    } else {
        let transfer_in = Transfer {
            from: ctx.accounts.user_token_b.to_account_info(),
            to: ctx.accounts.vault_b.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(CpiContext::new(cpi_program.clone(), transfer_in), amount_in)?;

        let transfer_out = Transfer {
            from: ctx.accounts.vault_a.to_account_info(),
            to: ctx.accounts.user_token_a.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let a_mint = ctx.accounts.mint_a.key();

        let b_mint = ctx.accounts.mint_b.key();
        let seeds: &[&[u8]] = &[b"authority", a_mint.as_ref(), b_mint.as_ref(), &[pool.bump]];
        token::transfer(
            CpiContext::new_with_signer(cpi_program.clone(), transfer_out, &[seeds]),
            amount_out_accum as u64,
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Exchange<'info> {
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
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"tick", pool.key().as_ref(), &i32::MIN.to_le_bytes()], bump)]
    pub tick_lower: Account<'info, Tick>,

    #[account(mut, seeds = [b"tick", pool.key().as_ref(), &i32::MAX.to_le_bytes()], bump)]
    pub tick_upper: Account<'info, Tick>,

    #[account(seeds = [b"authority", mint_a.key().as_ref(), mint_b.key().as_ref()], bump)]
    pub authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

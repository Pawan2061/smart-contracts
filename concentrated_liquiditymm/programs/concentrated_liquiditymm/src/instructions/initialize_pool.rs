use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::pool::Pool;
use crate::state::error::CLMMERROR;
use crate::utils::{price_to_sqrt_price_x64, sqrt_price_x64_to_tick};
pub fn init_pool(ctx: Context<InitializePool>,current_price:u64) -> Result<()> {
    require!(
        ctx.accounts.mint_a.key()!=ctx.accounts.mint_b.key(),
CLMMERROR::SameTokenMint        
    );

    
    let current_sqrt_price=price_to_sqrt_price_x64(current_price)?;

    let current_tick=sqrt_price_x64_to_tick(current_sqrt_price)?;


    let mut pool = ctx.accounts.pool.load_init()?;


    pool.mint_a=ctx.accounts.mint_a.key();
    pool.mint_b=ctx.accounts.mint_b.key();
    pool.vault_a=ctx.accounts.vault_a.key();

    pool.vault_b=ctx.accounts.vault_b.key();

    pool.lp_mint=ctx.accounts.lp_token_mint.key();


    pool.total_lp_issued=0;
    pool.bump=ctx.bumps.pool;

    pool.pool_authority=ctx.accounts.authority.key();
    pool.sqrt_price_x64=current_sqrt_price;

    pool.current_tick=current_tick;
    pool.active_liquidity=0;

    

    
    




    Ok(())
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"authority", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,


     #[account(
        init,
        payer = user,
        seeds = [b"lp_mint", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub lp_token_mint: Account<'info, Mint>,


    #[account(
        init,
        payer = user,
        seeds = [b"vault_token", mint_a.key().as_ref(), mint_b.key().as_ref(), b"A"],
        bump,
        token::mint = mint_a,
        token::authority = authority
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds = [b"vault_token", mint_a.key().as_ref(), mint_b.key().as_ref(), b"B"],
        bump,
        token::mint = mint_b,
        token::authority = authority
    )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + Pool::LEN, 
        seeds = [b"pool", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub pool: AccountLoader<'info, Pool>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

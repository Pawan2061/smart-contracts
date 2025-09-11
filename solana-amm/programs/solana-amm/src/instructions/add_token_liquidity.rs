use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

pub fn add_token_liquidity(
    ctx: Context<AddTokenLiquity>,
    quantity_a: u64,
    quantity_b: u64,
) -> Result<()> {
    require!(
        ctx.accounts.token_a_mint.key() == ctx.accounts.amm_pool.mint_a,
        SOLAMMERROR::InvalidTokenMint
    );
    require!(
        ctx.accounts.token_b_mint.key() == ctx.accounts.amm_pool.mint_b,
        SOLAMMERROR::InvalidTokenMint
    );
    require!(quantity_a > 0 && quantity_b > 0, SOLAMMERROR::ZeroAmount);

    let vault_a_balance = ctx.accounts.vault_a.amount;
    let vault_b_balance = ctx.accounts.vault_b.amount;
    let total_lp_supply = ctx.accounts.lp_token_mint.supply;

    msg!(
        "Reserves: A={}, B={} | LP Supply={}",
        vault_a_balance,
        vault_b_balance,
        total_lp_supply
    );

    let (lp_to_mint, used_a, used_b) = if total_lp_supply == 0 {
        let lp = integer_sqrt(quantity_a as u128 * quantity_b as u128);
        (lp as u64, quantity_a, quantity_b)
    } else {
        let lp_from_a = (quantity_a as u128)
            .checked_mul(total_lp_supply as u128)
            .unwrap()
            .checked_div(vault_a_balance as u128)
            .unwrap();

        let lp_from_b = (quantity_b as u128)
            .checked_mul(total_lp_supply as u128)
            .unwrap()
            .checked_div(vault_b_balance as u128)
            .unwrap();

        let lp_to_mint = lp_from_a.min(lp_from_b);

        let used_a = (lp_to_mint * vault_a_balance as u128) / total_lp_supply as u128;
        let used_b = (lp_to_mint * vault_b_balance as u128) / total_lp_supply as u128;

        (lp_to_mint as u64, used_a as u64, used_b as u64)
    };

    msg!(
        "Adding Liquidity → Used A={}, Used B={}, Minting LP={}",
        used_a,
        used_b,
        lp_to_mint
    );

    let cpi_accounts_a = Transfer {
        from: ctx.accounts.token_a_account.to_account_info(),
        to: ctx.accounts.vault_a.to_account_info(),
        authority: ctx.accounts.liquidity_provider.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_a),
        used_a,
    )?;

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.token_b_account.to_account_info(),
        to: ctx.accounts.vault_b.to_account_info(),
        authority: ctx.accounts.liquidity_provider.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_b),
        used_b,
    )?;

    let token_a_key = ctx.accounts.token_a_mint.key();
    let token_b_key = ctx.accounts.token_b_mint.key();

    let seeds: &[&[u8]] = &[
        b"authority",
        token_a_key.as_ref(),
        token_b_key.as_ref(),
        &[ctx.bumps.authority],
    ];
    let signer = &[seeds];

    let cpi_accounts_lp = MintTo {
        mint: ctx.accounts.lp_token_mint.to_account_info(),
        to: ctx.accounts.lp_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts_lp,
            signer,
        ),
        lp_to_mint,
    )?;

    let pool = &mut ctx.accounts.amm_pool;
    pool.total_lp_issued = pool.total_lp_issued.checked_add(lp_to_mint).unwrap();

    msg!("Liquidity added successfully!");
    Ok(())
}

fn integer_sqrt(value: u128) -> u128 {
    let mut x0 = value;
    let mut x1 = (x0 + 1) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + value / x0) / 2;
    }
    x0
}

#[derive(Accounts)]
pub struct AddTokenLiquity<'info> {
    pub liquidity_provider: Signer<'info>,

    #[account(
            seeds = [b"authority", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
            bump
        )]
    pub authority: UncheckedAccount<'info>,

    #[account(
            mut,
            associated_token::mint = token_a_mint,
            associated_token::authority = liquidity_provider
        )]
    pub token_a_account: Account<'info, TokenAccount>,

    #[account(
            mut,
            associated_token::mint = token_b_mint,
            associated_token::authority = liquidity_provider
        )]
    pub token_b_account: Account<'info, TokenAccount>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    #[account(
            mut,
            seeds = [b"vault_token", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), b"A"],
            bump,
            token::mint = token_a_mint,
            token::authority = authority
        )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
            mut,
            seeds = [b"vault_token", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), b"B"],
            bump,
            token::mint = token_b_mint,
            token::authority = authority
        )]
    pub vault_b: Account<'info, TokenAccount>,

    #[account(
            mut,
            seeds = [b"pool", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
            bump
        )]
    pub amm_pool: Account<'info, AMMPool>,

    #[account(
            mut,
            seeds = [b"lp_mint", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
            bump
        )]
    pub lp_token_mint: Account<'info, Mint>,

    #[account(
            mut,
            associated_token::mint = lp_token_mint,
            associated_token::authority = liquidity_provider
        )]
    pub lp_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct AMMPool {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub lp_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub total_lp_issued: u64,
    pub bump: u8,
}

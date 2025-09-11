use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    *,
};

pub fn add_token_liquidity(ctx: Context<AddTokenLiquity>) -> Result<()> {
    msg!("wokring add tokenmliquidty");
    Ok(())
}

#[derive(Accounts)]

pub struct AddTokenLiquity<'info> {
    pub liquidity_provider: Signer<'info>,

    #[account(seeds = [b"authority", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()], bump)]
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
         associated_token::mint = token_a_mint,
        associated_token::authority = liquidity_provider
    )]
    pub token_a_account: Account<'info, TokenAccount>,

    #[account(
        mut,
         associated_token::mint = token_a_mint,
        associated_token::authority = liquidity_provider
    )]
    pub token_b_account: Account<'info, TokenAccount>,

    #[account()]
    pub token_a_mint: Account<'info, Mint>,

    #[account()]
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
        seeds = [b"lp_mint", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump,
    )]

    pub lp_token_mint:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = lp_token_mint,
        associated_token::authority = liquidity_provider

    )]
    pub lp_token_account:Account<'info,TokenAccount>,








    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>
}

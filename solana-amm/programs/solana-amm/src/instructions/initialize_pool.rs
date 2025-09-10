use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};
pub fn initialize_pool() -> Result<()> {
    msg!("working frr");
    Ok(())
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds = [b"authority", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()], bump)]
    pub authority: UncheckedAccount<'info>,
    #[account()]
    pub token_a_mint: Account<'info, Mint>,

    #[account()]
    pub token_b_mint: Account<'info, Mint>,

   #[account(
        init,
        payer = payer,
        seeds = [b"vault_token", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), b"A"],
        bump,
        token::mint = token_a_mint, 
        token::authority = authority
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [b"vault_token", token_a_mint.key().as_ref(), token_b_mint.key().as_ref(), b"B"],
        bump,
        token::mint = token_b_mint,
        token::authority = authority
    )]
    pub vault_b: Account<'info, TokenAccount>,



      #[account(
        init,
        payer = payer,
        seeds = [b"lp_mint", token_a_mint.key().as_ref(), token_b_mint.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub lp_token_mint: Account<'info, Mint>,


    pub system_program: Program<'info,System>,
    pub token_program:Program<'info,Token>



}

// required things
// 1.payer
// 2.pool_authority
// 3.token_mint_a
// 4.token_mint_b
// 5.token_vault_a
// 6.token_vault_b
// 7.lp_mint
// 8.amm_pool

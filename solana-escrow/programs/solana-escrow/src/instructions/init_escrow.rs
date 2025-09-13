use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::EscrowError;
pub fn init_escrow(
    ctx: Context<InitEscrow>,
    amount_a: u64,  
    mint_b: Pubkey,
    amount_b: u64, 
) -> Result<()> {
    msg!(
        "Starting escrow for {} tokens of mint A, expecting {} tokens of mint B",
        amount_a,
        amount_b
    );

    let escrow = &mut ctx.accounts.escrow;

    escrow.initializer = ctx.accounts.initializer.key();
    escrow.vault = ctx.accounts.vault.key();
    escrow.mint_a = ctx.accounts.token_mint.key();
    escrow.amount_a = amount_a;
    escrow.mint_b = mint_b; 
    escrow.amount_b = amount_b;
    escrow.bump = ctx.bumps.escrow;
    escrow.is_active = true;

    require!(
        ctx.accounts.mint_token_account.amount >= amount_a,
        EscrowError::InsufficientBalance
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.mint_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount_a)?;

    msg!("Escrow initialized and Token A locked in vault");
    Ok(())
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = 8 + 200, 
        seeds = [b"escrow", initializer.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, EscrowState>,

    pub token_mint: Account<'info, Mint>, 

    #[account(
        mut,
        constraint = mint_token_account.mint == token_mint.key(),
        constraint = mint_token_account.owner == initializer.key()
    )]
    pub mint_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = token_mint,
        associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, anchor_spl::associated_token::AssociatedToken>,
}

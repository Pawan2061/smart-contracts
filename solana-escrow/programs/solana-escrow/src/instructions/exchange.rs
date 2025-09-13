use crate::state::EscrowError;
use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn exchange(ctx: Context<ExchangeToken>) -> Result<()> {
    let initializer_key = ctx.accounts.initializer.key();
    let mint_a_key = ctx.accounts.mint_a.key();

    let escrow = &mut ctx.accounts.escrow_state;

    require!(escrow.is_active, EscrowError::EscrowInactive);

    let cpi_accounts_b = Transfer {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        to: ctx.accounts.initializer_ata_b.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };
    let cpi_ctx_b = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_b);
    token::transfer(cpi_ctx_b, escrow.amount_b)?;

    let seeds = &[
        b"escrow",
        initializer_key.as_ref(),
        mint_a_key.as_ref(),
        &[escrow.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_a = Transfer {
        from: ctx.accounts.escrow_vault_a.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: escrow.to_account_info(),
    };
    let cpi_ctx_a = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts_a,
        signer_seeds,
    );
    token::transfer(cpi_ctx_a, escrow.amount_a)?;

    escrow.is_active = false;

    Ok(())
}

#[derive(Accounts)]
pub struct ExchangeToken<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub initializer: SystemAccount<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = initializer,
    )]
    pub initializer_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = escrow_vault_a.mint == mint_a.key(),
        constraint = escrow_vault_a.key() == escrow_state.vault,
        constraint = escrow_vault_a.owner == escrow_state.key(),
    )]
    pub escrow_vault_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"escrow", initializer.key().as_ref(), mint_a.key().as_ref()],
        bump = escrow_state.bump,
    )]
    pub escrow_state: Account<'info, EscrowState>,

    pub token_program: Program<'info, Token>,
}

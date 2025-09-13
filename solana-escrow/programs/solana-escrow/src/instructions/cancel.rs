use crate::state::{EscrowError, EscrowState};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
    let initializer_key = ctx.accounts.initializer.key();
    let mint_a_key = ctx.accounts.mint_a.key();

    let escrow = &mut ctx.accounts.escrow_state;

    require!(escrow.is_active, EscrowError::EscrowInactive);

    let seeds = &[
        b"escrow",
        initializer_key.as_ref(),
        mint_a_key.as_ref(),
        &[escrow.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.escrow_vault.to_account_info(),
        to: ctx.accounts.initializer_ata.to_account_info(),
        authority: escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    token::transfer(cpi_ctx, escrow.amount_a)?;

    escrow.is_active = false;

    msg!("Escrow cancelled: Token A returned to initializer");

    Ok(())
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut)]
    pub mint_a: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = initializer,
    )]
    pub initializer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = escrow_vault.mint == mint_a.key(),
        constraint = escrow_vault.owner == escrow_state.key(),
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"escrow", initializer.key().as_ref(), mint_a.key().as_ref()],
        bump = escrow_state.bump,
    )]
    pub escrow_state: Account<'info, EscrowState>,

    pub token_program: Program<'info, Token>,
}

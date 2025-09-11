use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};

use crate::state::*;

pub fn swap_token(
    ctx: Context<SwapTokenContext>,
    amount_in: u64,
    min_amount_out: u64,
    is_a_to_b: bool, // true if swapping A → B, false if B → A
) -> Result<()> {
    // 1️⃣ Validate input
    require!(amount_in > 0, SOLAMMERROR::ZeroAmount);

    // 2️⃣ Determine which vaults and user accounts are input/output
    let (vault_in, vault_out, user_in, user_out) = if is_a_to_b {
        (
            &ctx.accounts.vault_a,
            &ctx.accounts.vault_b,
            &ctx.accounts.token_account_a,
            &ctx.accounts.token_account_b,
        )
    } else {
        (
            &ctx.accounts.vault_b,
            &ctx.accounts.vault_a,
            &ctx.accounts.token_account_b,
            &ctx.accounts.token_account_a,
        )
    };

    // 3️⃣ Calculate output using constant product formula
    // dy = (y * dx) / (x + dx)
    let amount_out = vault_out
        .amount
        .checked_mul(amount_in)
        .and_then(|p| p.checked_div(vault_in.amount.checked_add(amount_in)?))
        .ok_or(SOLAMMERROR::ArithmeticOverflow)?;

    require!(amount_out >= min_amount_out, SOLAMMERROR::SlippageExceeded);

    // 4️⃣ Transfer input tokens from user to vault
    let cpi_accounts_in = Transfer {
        from: user_in.to_account_info(),
        to: vault_in.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program.clone(), cpi_accounts_in), amount_in)?;

    // 5️⃣ Transfer output tokens from vault to user using PDA signer

    let token_a_key=ctx.accounts.token_mint_a.key();
        let token_b_key=ctx.accounts.token_mint_b.key();

    
    let seeds = &[
        b"authority",
        token_a_key.as_ref(),
        token_b_key.as_ref(),
        &[ctx.bumps.authority],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts_out = Transfer {
        from: vault_out.to_account_info(),
        to: user_out.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts_out, signer_seeds),
        amount_out,
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct SwapTokenContext<'info> {
    #[account()]
    pub signer: Signer<'info>,

    #[account()]
    pub token_mint_a: Account<'info, Mint>,

    #[account()]
    pub token_mint_b: Account<'info, Mint>,


      #[account(
            seeds = [b"authority", token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
            bump
        )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = signer,
    )]
    pub token_account_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = signer,

    )]
    pub token_account_b: Account<'info, TokenAccount>,

    #[account(
            mut,
            seeds = [b"vault_token", token_mint_a.key().as_ref(), token_mint_b.key().as_ref(), b"A"],
            bump,
        )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
            mut,
            seeds = [b"vault_token", token_mint_a.key().as_ref(), token_mint_b.key().as_ref(), b"B"],
            bump,
            
    )]
    pub vault_b: Account<'info, TokenAccount>,


    #[account(
        mut,
        seeds = [b"pool", token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        bump
    )]
    pub amm_pool:Account<'info,AMMPool>,
    pub token_program:Program<'info,Token>
}



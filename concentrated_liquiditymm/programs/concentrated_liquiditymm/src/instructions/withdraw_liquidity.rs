use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub fn withdraw_liquidity() -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct WithdrawLiquidity<'info> {
    pub token_program: Program<'info, Token>,
}

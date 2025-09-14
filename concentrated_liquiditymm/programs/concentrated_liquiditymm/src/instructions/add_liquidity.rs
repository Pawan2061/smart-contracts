use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub fn add_liquidity() -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct AddLiquidity<'info> {
    pub token_program: Program<'info, Token>,
}

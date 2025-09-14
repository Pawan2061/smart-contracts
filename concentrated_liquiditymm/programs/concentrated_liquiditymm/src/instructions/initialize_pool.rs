use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub fn init_pool() -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct InitializePool<'info> {
    pub token_program: Program<'info, Token>,
}

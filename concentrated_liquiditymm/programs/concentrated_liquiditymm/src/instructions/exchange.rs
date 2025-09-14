use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub fn exchange() -> Result<()> {
    msg!("wokring");
    Ok(())
}

#[derive(Accounts)]

pub struct Exchange<'info> {
    pub token_program: Program<'info, Token>,
}

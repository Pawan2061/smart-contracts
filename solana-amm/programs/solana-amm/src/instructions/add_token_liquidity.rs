use anchor_lang::prelude::*;
use anchor_spl::*;

pub fn add_token_liquidity(ctx: Context<AddTokenLiquity>) -> Result<()> {
    msg!("wokring add tokenmliquidty");
    Ok(());
}

#[derive(Accounts)]

pub struct AddTokenLiquity<'info> {
    #[account()]
    pub liquidity_provider: Signer<'info>,
}

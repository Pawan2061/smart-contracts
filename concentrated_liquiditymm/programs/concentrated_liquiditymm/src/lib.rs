use anchor_lang::prelude::*;

declare_id!("7bo78Lu7uofN94e4Z6aJ1xnni1Brg32qn3Ut6fmcrkj5");

#[program]
pub mod concentrated_liquiditymm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

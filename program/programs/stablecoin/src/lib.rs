use anchor_lang::prelude::*;

declare_id!("H1CQzYo1R6Qog6YUeK5wBPErSVD4FNumwGMvVCJxyn5s");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

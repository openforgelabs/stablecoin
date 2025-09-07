use anchor_lang::prelude::*;\
use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;


declare_id!("H1CQzYo1R6Qog6YUeK5wBPErSVD4FNumwGMvVCJxyn5s");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

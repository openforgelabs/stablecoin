use anchor_lang::prelude::*;\
use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;
use error::*;
mod error;


declare_id!("H1CQzYo1R6Qog6YUeK5wBPErSVD4FNumwGMvVCJxyn5s");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx , min_health_factor)
    }

    pub fn process_deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) => Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx , amount_collateral , amount_to_mint)
    }
}



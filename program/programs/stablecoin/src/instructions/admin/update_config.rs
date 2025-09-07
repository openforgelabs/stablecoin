use anchor_lang::prelude::*;
use craet::{Config , SEED_CONFIG_ACCOUNT};

#[derive(Accounts)]

pub struct UpdateConfig<'info> {
    #[account(
        mut , seeds= [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
}

pub fn process_update_config(cyx: Context<InitializeConfig>) -> Result<()> {
    process_update_config(ctx , min_health_factor)
}
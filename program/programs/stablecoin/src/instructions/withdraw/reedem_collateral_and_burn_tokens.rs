use anchor_lang::prelude::*;
use pyth_solana_reciever_sdk::price_update::PriceUpdateV2;
use crate::{Collateral , Config ,SEED_COLLATERAL_ACCOUNT , SEED_CONFIG_ACCOUNT , SEED_SOL_ACCOUNT};

#[derive(Accounts)]

pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositer : Signer<'info>,

    pub price_update: Account<'info , PriceUpdateV2>,
    #[account(
        seeds = [SEED_COLLATERAL_ACCOUNT],
        bump = config.account.bump,
        has_one= mint_account,
    )]

    pub config_account: Account<'info , Config>,

    #[account(
        mut, 
        seeeds = [SEED_COLLATERAL_ACCOUNT , depositer.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account,
    )]

    pub collateral_account: Account<'info , Collateral>,
    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub min_account: InterfaceAccount<'info , Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info , System>,
    pub token_program: Program<'info, Token2022>,
}


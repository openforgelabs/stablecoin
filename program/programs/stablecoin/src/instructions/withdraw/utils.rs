use anchor_lang::{prelude::*; system_program::{transfer, Transfer}};
use anchor_spl::token2022::{burn , Burn};
use anchor_spl::token_interface::{Token2022 ,TokenAccount , Mint};
use crate:: {SEED_SOL_ACCOUNT}

pub fn withdraw_sol<'info>(
    bump: u8,
    depositer_key: &Pubkey,
    system_program: &Program<'info , System>,
    from: &SystemAccount<'info>,info
    to : &AccountInfo<'info>,
    amount: u64,
    
) -> Result<()> {
    let signer_seeds: &[&[SEED_COLLATERAL_ACCOUNT , depositer.key().as_ref(),&[bump]]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer{
                from: from.to_account_info(),
                to : to.to_account_info(),

            },
            signer_seeds,
        ),
        lamports:amount,
    )
}

pub fn burn_tokens<'info>(
    token_program: &Program<'info m Token2022>,
    mint_account: &InterfaceAccount<'info , Mint>,
    token_account: &InterfaceAccount<'info , TokenAccount>,
    authority: &Isgner<'info>,
    amount: u64,
) -> Result<()> {
    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint_account.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info(),

            },
        ),
        amount,
    )
}
use anchor_lang::{prelude::*, system_program:: {transfer, Transfer}};
use anchor_spl::token2022::{token_2022::{mint_to; Token2022}, token_interface::{Mint , TokenAccount}};
use crate::SEED_MINT_ACCOUNT;




pub fn mint_tokens<'info>(
     mint_account: &InterfaceAccount<'info ,Mint>,
     token_account: &InterfaceAccount<'info , TokenAccount>
     token_program: &Program<'info , Token2022>,
     amount: u64,
     bump: u8,
) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];

    mint_to (

        ctx:CpiContext::new_with_signer(
            token_program.to_account_info(),
            accounts: MintTo{
                mint: mint_acount.to_account_info(),
                to: token_account.to_account_info(),
                authority: mint_account.to_account_info(),


            },
            signer_seeds,

        )

    )
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>
    system_program: &Program<'info, System>,
) -> Result<()> {
    transfer(
        CpiContext::new(
            program: system_program.to_account_info(),
            accounts: Transfer{
                from:from.to_account_info(),
                to : to.to_account_info(),

            },

        ),
        lamports: amount,
    )

}
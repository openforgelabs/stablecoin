use anchor_lang::prelude::*;



pub struct Collateral  {
    pub deposit : Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub lamport_balance: u64,
    pub amount_minted : u64,
    pub bump : u8,
    pub bump_sol_account: u8,
    pub is_initilized: bool, 
}
#[account]
#derive[InitSpace , Debug]

pub struct Config {
    pub athority: Pubkey,
    pub min_account : Pubkey,
    pub liquidation_bonus: u64,
    pub liquidation_threshold: u64,
    pub min_health_factor: u64,
    pub bump_mint_account : u8,
}
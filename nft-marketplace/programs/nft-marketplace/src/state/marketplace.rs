use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u64,
    pub bump: u8,
    pub treasury_bump: u8,
    #[max_len(40)]
    pub name: String,
}

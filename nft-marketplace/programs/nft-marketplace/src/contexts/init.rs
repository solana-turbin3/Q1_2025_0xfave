use anchor_lang::prelude::*;

use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = 8 + Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(seeds = [b"treasury", marketplace.key().as_ref()], bump)]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, fee: u64, name: String, bump: InitializeBumps) -> Result<()> {
        require!(name.len() <= 32, InitializeError::NameTooLong);
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bump.marketplace,
            treasury_bump: bump.treasury,
            name,
        });
        
        Ok(())
    }
}


// errors
#[error_code]
pub enum InitializeError {
    #[msg("Name too long")]
    NameTooLong,
}

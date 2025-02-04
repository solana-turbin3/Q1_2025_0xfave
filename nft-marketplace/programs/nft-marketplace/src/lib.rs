use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

pub mod instructions;

declare_id!("7JZmtWzBZBgSzwpsX3P4nQHidYSUCf4Twa6zR6ro2bVM");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<Initialize>, fee: u64, name: String) -> Result<()> {
        ctx.accounts.init(fee, name, ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        instructions::list(ctx, price)
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        instructions::delist(ctx)
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        instructions::purchase(ctx)
    }
}

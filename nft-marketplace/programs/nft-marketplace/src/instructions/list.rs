use anchor_lang::prelude::*;

use crate::List;

pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
    ctx.accounts.create_listing(price, ctx.bumps)?;
    ctx.accounts.deposit_nft()
}

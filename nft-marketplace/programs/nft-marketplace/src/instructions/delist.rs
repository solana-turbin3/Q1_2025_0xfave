use anchor_lang::prelude::*;

use crate::Delist;

pub fn delist(ctx: Context<Delist>) -> Result<()> {
    ctx.accounts.withdraw_nft()?;
    ctx.accounts.close_account()?;
    Ok(())
}

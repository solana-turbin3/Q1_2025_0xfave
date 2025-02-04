use anchor_lang::prelude::*;

use crate::Purchase;

pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    ctx.accounts.transfer_sol()?;
    ctx.accounts.withdraw_nft()?;
    ctx.accounts.close_listing()?;
    Ok(())
}

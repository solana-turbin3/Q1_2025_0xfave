use anchor_lang::prelude::*;

use crate::contexts::refund::Refund;

pub fn refund(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.refund_and_close_vault()?;
    Ok(())
}

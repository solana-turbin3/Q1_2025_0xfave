use anchor_lang::prelude::*;

use crate::contexts::take::Take;

pub fn take(ctx: Context<Take>) -> Result<()> {
    ctx.accounts.deposit()?;
    ctx.accounts.withdraw_and_close()?;

    Ok(())
}

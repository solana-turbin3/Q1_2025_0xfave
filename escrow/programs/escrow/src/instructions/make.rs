use anchor_lang::prelude::*;

use crate::contexts::make::Make;

/// Instruction to create an escrow and deposit tokens.
///
/// # Arguments
/// - `ctx`: Context containing accounts for the `make` instruction.
/// - `seed`: A unique seed for deriving the escrow account address.
/// - `deposit`: The amount of tokens to deposit into the escrow.
/// - `receive_amount`: The amount of tokens the maker expects to receive.
pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive_amount: u64) -> Result<()> {
    // Validate inputs
    require!(deposit > 0, EscrowError::InvalidDepositAmount);
    require!(receive_amount > 0, EscrowError::InvalidReceiveAmount);

    ctx.accounts
        .create_escrow(seed, receive_amount, &ctx.bumps)?;
    ctx.accounts.deposit(deposit)?;

    msg!(
        "Escrow created with seed {} and deposited {} into the vault.",
        seed,
        deposit
    );

    Ok(())
}

#[error_code]
pub enum EscrowError {
    #[msg("Invalid receive amount.")]
    InvalidReceiveAmount,
    #[msg("Invalid deposit amount.")]
    InvalidDepositAmount,
}

use anchor_lang::prelude::*;

/// Represents an escrow account for holding funds during a transaction.
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    /// The seed for the escrow account
    pub seed: u64,
    /// The public key of the maker of the escrow.
    pub maker: Pubkey,

    /// The mint associated with the deposit token.
    pub deposit_mint: Pubkey,

    /// The mint associated with the token to be received.
    pub receive_mint: Pubkey,

    /// The amount of tokens to be received.
    pub receive_amount: u64,

    /// A bump seed for ensuring unique addresses.
    pub bump: u8,
}

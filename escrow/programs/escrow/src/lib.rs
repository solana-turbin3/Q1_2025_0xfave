use anchor_lang::prelude::*;

declare_id!("2EEvRMjqtcpoSgrD9aZja6zKhtyD7HTK44qL6o1wdfjo");

pub mod contexts;
pub mod state;
pub use contexts::*;
pub mod instructions;

#[program]
pub mod escrow {
    use super::*;

    /// Initializes a new escrow and deposits tokens.
    ///
    /// # Arguments
    /// - `ctx`: Context containing accounts for the `make` instruction.
    /// - `seed`: A unique seed for deriving the escrow account address.
    /// - `deposit`: The amount of tokens to deposit into the escrow.
    /// - `receive`: The amount of tokens the maker expects to receive.
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        instructions::make::make(ctx, seed, deposit, receive)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::take(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::refund(ctx)
    }
}

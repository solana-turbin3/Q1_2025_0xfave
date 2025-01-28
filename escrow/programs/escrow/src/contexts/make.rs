use crate::state::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    /// The user initiating the escrow who signs the transaction.
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The token mint for the tokens being deposited into the escrow.
    #[account(mint::token_program = token_program)]
    pub deposit_mint: InterfaceAccount<'info, Mint>,

    /// The token mint for the tokens the maker expects to receive.
    #[account(mint::token_program = token_program)]
    pub receive_mint: InterfaceAccount<'info, Mint>,

    /// The maker's associated token account for the deposit mint.
    #[account(
        mut,
        associated_token::mint = deposit_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub deposit_mint_ata: InterfaceAccount<'info, TokenAccount>,

    /// The escrow account that will store the state of the escrow.
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_be_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// The vault account that will hold the deposited tokens.
    #[account(
        init,
        payer = maker,
        associated_token::mint = deposit_mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The SPL Associated Token Program.
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// The SPL Token Program.
    pub token_program: Interface<'info, TokenInterface>,

    /// The Solana System Program.
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    /// Initializes the escrow account with the provided data.
    pub fn create_escrow(
        &mut self,
        seed: u64,
        receive_amount: u64,
        bumps: &MakeBumps,
    ) -> Result<()> {
        // Initialize the escrow account
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.deposit_mint.key(),
            mint_b: self.receive_mint.key(),
            receive_amount,
            bump: bumps.escrow,
        });

        // Emit an event
        emit!(EscrowCreated {
            escrow: self.escrow.key(),
            maker: self.maker.key(),
            seed,
            receive_amount,
        });

        Ok(())
    }

    /// Transfers tokens from the maker's ATA to the escrow's vault.
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        // Transfer tokens
        let transfer = TransferChecked {
            from: self.deposit_mint_ata.to_account_info(),
            mint: self.deposit_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer);
        transfer_checked(cpi_ctx, deposit, self.deposit_mint.decimals)?;

        // Emit an event
        emit!(TokensDeposited {
            maker: self.maker.key(),
            escrow: self.escrow.key(),
            deposit,
        });

        Ok(())
    }
}

/// Event emitted when an escrow is created.
#[event]
pub struct EscrowCreated {
    pub maker: Pubkey,
    pub escrow: Pubkey,
    pub seed: u64,
    pub receive_amount: u64,
}

/// Event emitted when tokens are deposited into the escrow.
#[event]
pub struct TokensDeposited {
    pub maker: Pubkey,
    pub escrow: Pubkey,
    pub deposit: u64,
}

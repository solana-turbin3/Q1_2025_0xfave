use anchor_lang::prelude::*;

declare_id!("XGEYP46XmmLvr1KxgJX3pANGrAFsxhafwEbncvhWPsv");

#[program]
pub mod vault {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing Vault from: {:?}", ctx.program_id);
        let vault = &mut ctx.accounts.vault;

        // check that the vault has not be initialized before
        if vault.owner != Pubkey::default() {
            return Err(ErrorCode::VaultAlreadyInitialized.into());
        }

        // initialize the vault owner
        vault.owner = ctx.accounts.owner.key();

        // initialize the vault total amount
        vault.total_amount = 0;

        // set initialization flag
        vault.is_initialized = true;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        msg!("Depositing to: {:?}", ctx.program_id);

        // get sender's balance
        let sender_balance = ctx.accounts.sender.lamports();
        let rent = Rent::get()?;
        let rent_exempt_min = rent.minimum_balance(0);

        // make sure is enough to send out (consider rent exempt fee)
        if sender_balance < amount + rent_exempt_min {
            return Err(ErrorCode::InsuficientFunds.into());
        }

        // transfer sol to vault
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                Transfer {
                    from: ctx.accounts.sender.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        
        // update vault total amount
        ctx.accounts.vault.total_amount += amount;

        msg!("Deposited {} lamports into the vault.", amount);
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        msg!("Withdrawing from: {:?}", ctx.program_id);

        // Verify the vault owner
        let vault = &ctx.accounts.vault;
        if vault.owner != ctx.accounts.owner.key() {
            return Err(ErrorCode::Unauthorized.into());
        }

        msg!("Vault Owner: {}", ctx.accounts.vault.owner);
        msg!("Instruction Owner: {}", ctx.accounts.owner.key());
        msg!("Vault Balance: {}", ctx.accounts.vault.total_amount);
        msg!("Withdraw Amount: {}", amount);

        // check vault balance
        let vault = &ctx.accounts.vault;
        if vault.total_amount < amount {
            return Err(ErrorCode::InsuficientVaultFunds.into());
        }

        // update the total amount
        ctx.accounts.vault.total_amount -= amount;

        // transfer the sol to the sender
        // transfer(
        //     CpiContext::new(
        //         ctx.accounts.system_program.to_account_info(), 
        //         Transfer {
        //             from: ctx.accounts.vault.to_account_info(),
        //             to: ctx.accounts.owner.to_account_info(),
        //         },
        //     ),
        //     amount,
        // )?;

        // TODO why did this work and not the one on top?
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += amount;
        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;

        Ok(())
    }
}

#[account]
pub struct Vault {
    pub total_amount: u64, // Total amount in the vault
    pub owner: Pubkey, // Owner of the vault
    pub is_initialized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The vault has already been initialized")]
    VaultAlreadyInitialized,
    #[msg("The sender does not have enough SOL to deposit.")]
    InsuficientFunds,
    #[msg("The vault does not have enough SOL to cover withdrawal.")]
    InsuficientVaultFunds,
    #[msg("Only the vault owner can withdraw")]
    Unauthorized,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 32 + 1)] // TODO explain this as a tweet 8 for the space for the key, 8 for the amount and 32 for the pubkey
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner : Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

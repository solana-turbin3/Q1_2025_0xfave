use std::str::FromStr;

use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::{
        commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::{read_keypair_file, Keypair}, signer::Signer, system_program
    }, Client, Cluster
};
use vault::Vault;
use std::sync::Arc;

fn setup() -> (Keypair, Arc<Keypair>, Keypair, Keypair, Pubkey, Client<Arc<Keypair>>) {
    // Load environment variables from .env file
    dotenv::dotenv().expect("Failed to load .env file");
    let program_id = "XGEYP46XmmLvr1KxgJX3pANGrAFsxhafwEbncvhWPsv"; // Replace with your program ID
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());

    let client = Client::new_with_options(Cluster::Localnet, payer.clone(), CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // Create wallets for alice, and bob
    let alice = Keypair::new();
    let bob = Keypair::new();

    // Fund the wallets using the Solana faucet
    let rpc_url = "http://localhost:8899"; // Localnet RPC URL
    let rpc_client = RpcClient::new(rpc_url);

    // Fund alice
    let fund_amount = LAMPORTS_PER_SOL * 4; // 4 SOL
    let signature = rpc_client
        .request_airdrop(&alice.pubkey(), fund_amount)
        .expect("Failed to fund alice from faucet");

    // Wait for the airdrop to confirm
    rpc_client
        .confirm_transaction(&signature)
        .expect("Failed to confirm airdrop for alice");

    println!("Funded alice with 1 SOL from faucet.");

    // Fund bob
    let signature = rpc_client
        .request_airdrop(&bob.pubkey(), fund_amount)
        .expect("Failed to fund bob from faucet");

    // Wait for the airdrop to confirm
    rpc_client
        .confirm_transaction(&signature)
        .expect("Failed to confirm airdrop for bob");

    println!("Funded bob with 1 SOL from faucet.");

    // Initialize the vault using the payer
    let vault_keypair = Keypair::new();

    let tx = program
        .request()
        .accounts(vault::accounts::Initialize {
            vault: vault_keypair.pubkey(),
            owner: payer.pubkey(),
            system_program: system_program::id(),
        })
        .args(vault::instruction::Initialize {})
        .signer(&vault_keypair)
        .signer(&payer)
        .send()
        .expect("Failed to initialize vault");

    println!("Vault initialized. Transaction signature: {}", tx);

    // Return the vault keypair, wallets, program ID, and client for reuse
    (vault_keypair, payer, alice, bob, program_id, client)
}

fn deposit(
    program: &anchor_client::Program<Arc<Keypair>>,
    vault_pubkey: Pubkey,
    sender: &Keypair,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Deposit SOL into the vault
    let deposit_tx = program
        .request()
        .accounts(vault::accounts::Deposit {
            vault: vault_pubkey,          // Vault account
            sender: sender.pubkey(),     // Sender account
            system_program: system_program::id(), // System program
        })
        .args(vault::instruction::Deposit { amount })
        .signer(sender) // Only the sender needs to sign
        .send()?;

    println!(
        "{} deposited {} lamports. Transaction signature: {}",
        sender.pubkey(),
        amount,
        deposit_tx
    );

    Ok(())
}

fn withdraw(
    program: &anchor_client::Program<Arc<Keypair>>,
    vault_keypair: &Keypair,
    owner: &Keypair,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = program
        .request()
        .accounts(vault::accounts::Withdraw {
            vault: vault_keypair.pubkey(),
            owner: owner.pubkey(),
            system_program: system_program::id(),
        })
        .args(vault::instruction::Withdraw { amount })
        .signer(owner)
        .send()?;

    println!(
        "{} withdrew {} lamports. Transaction signature: {}",
        owner.pubkey(),
        amount,
        tx
    );

    Ok(())
}

#[test]
fn test_initialize() {
    // Use the setup function to initialize the environment
    let (vault_keypair, creator, _, _, program_id, client) = setup();

    let program = client.program(program_id).unwrap();

    // Attempt to reinitialize the vault (should fail)
    let result = program
        .request()
        .accounts(vault::accounts::Initialize {
            vault: vault_keypair.pubkey(),
            owner: creator.pubkey(),
            system_program: system_program::ID,
        })
        .args(vault::instruction::Initialize {})
        .signer(&vault_keypair)
        .signer(&creator)
        .send();

    // Check that the reinitialization fails
    assert!(result.is_err(), "Vault was reinitialized, but it should have failed.");

}

#[test]
fn test_deposit() -> Result<(), Box<dyn std::error::Error>> {
    let (vault_keypair, creator, _, _, program_id, client) = setup();

    let program = client.program(program_id).unwrap();

    // deposit SOL into the vault
    deposit(&program, vault_keypair.pubkey(), &creator, 1_000_000_000)?;


    // Fetch the vault account and verify the total amount
    let vault_account = program.account::<Vault>(vault_keypair.pubkey()).unwrap();
    assert_eq!(vault_account.total_amount, 1_000_000_000);
    println!("Vault total amount: {}", vault_account.total_amount);

    Ok(())
}

#[test]
fn test_multiple_deposits() -> Result<(), Box<dyn std::error::Error>> {
    // Use the setup function to initialize the environment
    let (vault_keypair, creator, alice, bob, program_id, client) = setup();

    let program = client.program(program_id).unwrap();
    let vault_pubkey = vault_keypair.pubkey();

    // Deposit from creator
    deposit(&program, vault_pubkey, &creator, 500_000_000)?; // 0.5 SOL

    // Deposit from alice
    deposit(&program, vault_pubkey, &alice, 1_000_000_000)?; // 1 SOL

    // Deposit from bob
    deposit(&program, vault_pubkey, &bob, 750_000_000)?; // 0.75 SOL

    // Fetch the vault account and verify the total amount
    let vault_account = program.account::<Vault>(vault_pubkey)?;
    let expected_total_amount = 500_000_000 + 1_000_000_000 + 750_000_000;
    assert_eq!(vault_account.total_amount, expected_total_amount);
    println!(
        "Vault total amount after all deposits: {} lamports",
        vault_account.total_amount
    );

    Ok(())
}

#[test]
fn test_withdraw_as_owner() -> Result<(), Box<dyn std::error::Error>> {
    // Use the setup function to initialize the environment
    let (vault_keypair, creator, _, _, program_id, client) = setup();

    let program = client.program(program_id).unwrap();
    let vault_pubkey = vault_keypair.pubkey();

    // Deposit SOL into the vault (for testing)
    deposit(&program, vault_pubkey, &creator, 1_000_000_000)?; // 1 SOL

    // Withdraw SOL from the vault as the owner
    let withdraw_amount = 500_000_000; // 0.5 SOL
    withdraw(&program, &vault_keypair, &creator, withdraw_amount)?;

    // Fetch the vault account and verify the total amount
    let vault_account = program.account::<Vault>(vault_pubkey)?;
    assert_eq!(vault_account.total_amount, 1_000_000_000 - withdraw_amount);
    println!("Vault total amount after withdrawal: {} lamports", vault_account.total_amount);

    Ok(())
}

#[test]
fn test_withdraw_as_non_owner() -> Result<(), Box<dyn std::error::Error>> {
    // Use the setup function to initialize the environment
    let (vault_keypair, creator, alice, _, program_id, client) = setup();

    let program = client.program(program_id).unwrap();
    let vault_pubkey = vault_keypair.pubkey();

    // Deposit SOL into the vault (for testing)
    deposit(&program, vault_pubkey, &creator, 1_000_000_000)?; // 1 SOL

    // Attempt to withdraw SOL from the vault as a non-owner (alice)
    let withdraw_amount = 500_000_000; // 0.5 SOL
    let result = withdraw(&program, &vault_keypair, &alice, withdraw_amount);

    // Check that the withdrawal fails
    assert!(result.is_err(), "Non-owner was able to withdraw from the vault.");

    // Fetch the vault account and verify the total amount (should remain unchanged)
    let vault_account = program.account::<Vault>(vault_pubkey)?;
    assert_eq!(vault_account.total_amount, 1_000_000_000);
    println!("Vault total amount after failed withdrawal: {} lamports", vault_account.total_amount);

    Ok(())
}

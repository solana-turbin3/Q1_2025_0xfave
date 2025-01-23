use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::io::{self, BufRead};
mod programs;
mod read_keypair_file;
mod send;
mod transfer;
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};
use solana_sdk::{
    signature::read_keypair_file,
    transaction::Transaction,
    system_program,
};

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    let rpc_client = RpcClient::new(RPC_URL);
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Let's define our accounts
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

    let prereq = Turbin3PrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let args = CompleteArgs {
        github: b"0xfave".to_vec(),
    };

    // Get recent blockhash
    let blockhash = rpc_client.get_latest_blockhash().expect(
        "Failed to get recent
blockhash",
    );

    // Now we can invoke the "complete" function 
    let transaction = Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction");

    println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}

// let kp = Keypair::new(); A4D1bZpxMP55RwFJWhRHVsySS9VLLnFkBNQxPh4HbTs3

#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");

    println!("{:?}", kp.to_bytes());
}

#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array:");
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    println!("Your private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}

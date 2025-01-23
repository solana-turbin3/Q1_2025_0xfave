use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};

use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    // Import our keypair
    let keypair: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Define our Turbin3 public key
    let to_pubkey = Pubkey::from_str("6U1sHmKDuN8rPj4UurgGL5yaVsahFuLadDg7LP9HRTAZ").unwrap();

    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

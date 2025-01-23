use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};

const RPC_URL: &str = "https://api.devnet.solana.com";

// Import our keypair
fn main() {
    let keypair: Keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);

    // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=devnet",
                s.to_string()
            );
        }

        Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
    };
}

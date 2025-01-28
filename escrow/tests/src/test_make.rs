use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    },
    Client, Cluster,
};

#[test]
fn test_make() {
    let program_id = "2EEvRMjqtcpoSgrD9aZja6zKhtyD7HTK44qL6o1wdfjo";
    let payer = Keypair::new();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    let tx = program
        .request()
        .accounts(escrow::accounts::Make {
            maker: payer.pubkey(),
            deposit_mint: todo!(),
            receive_mint: todo!(),
            deposit_mint_ata: todo!(),
            escrow: todo!(),
            vault: todo!(),
            associated_token_program: todo!(),
            token_program: todo!(),
            system_program: todo!(),
        })
        .args(escrow::instruction::Make {
            seed: todo!(),
            deposit: todo!(),
            receive: todo!(),
        })
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}

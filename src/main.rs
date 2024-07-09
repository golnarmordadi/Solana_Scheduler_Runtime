use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::sync::Arc;
use tokio::time::{self, Duration};

async fn send_transaction(client: Arc<RpcClient>, payer: Arc<Keypair>, recipient: Pubkey, amount: u64) {
    let recent_blockhash = client.get_recent_blockhash().expect("Failed to get recent blockhash").0;

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&payer.pubkey(), &recipient, amount)],
        Some(&payer.pubkey()),
        &[&*payer],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(signature) => println!("Transaction succeeded: {}", signature),
        Err(err) => eprintln!("Transaction failed: {}", err),
    }
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = Arc::new(RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed()));
    
    let payer = Arc::new(Keypair::new());
    let recipient = Pubkey::from_str("RecipientPubkeyHere").unwrap();
    let amount = 1_000_000; // Amount in lamports (0.001 SOL)

    let mut interval = time::interval(Duration::from_secs(10)); // Schedule every 10 seconds

    loop {
        interval.tick().await;
        send_transaction(client.clone(), payer.clone(), recipient, amount).await;
    }
}

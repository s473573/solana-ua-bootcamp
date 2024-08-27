// Import necessary dependencies
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use spl_token::instruction::transfer;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let secret = env::var("SECRET_KEY")
        .expect("Add SENDER_SECRET_KEY to .env!");

    let peer_secret = env::var("PEER_SECRET_KEY")
        .expect("This program requires an additional keypair.");

    let secret_vals: Vec<u8> = serde_json::from_str(&secret)?;
    let peer_secret_vals: Vec<u8> = serde_json::from_str(&peer_secret)?;

    let own = Keypair::from_bytes(&secret_vals)?;
    let peer = Keypair::from_bytes(&peer_secret_vals)?;

    let conn = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Sender's public key is: {}", own.pubkey());
    println!("🔑 Recipient's public key is: {}", peer.pubkey());

    let token_account = Pubkey::from_str("6Rx2RhLdxowbGcvNR3DMimR1od8XsmDjdzAMZCkkDYzM")?;
    let peer_token_account = Pubkey::from_str("DLueJ8GKRjJxgMhEV3HUaTmiSB9QhEUWR5DZW5hCGnmw")?;

    let amount = 100_u64;

    let transfer_ix = transfer(
        &spl_token::id(),
        &token_account,
        &peer_token_account,
        &own.pubkey(),
        &[],
        amount,
    )?;

    let mut t = Transaction::new_with_payer(
        &[transfer_ix],
        Some(&peer.pubkey()), // payment is on the recipient this time
    );

    // obtaining the required signature
    let recent_blockhash = conn.get_latest_blockhash()?;
    t.sign(&[&own, &peer], recent_blockhash);

    println!("🔑 Each actor has signed the transaction.");

    // Send and confirm the transaction
    let sign = conn.send_and_confirm_transaction(&t)?;

    // Print the transaction signature
    println!("✅ Transaction confirmed, signature: {}", sign);

    Ok(())
}


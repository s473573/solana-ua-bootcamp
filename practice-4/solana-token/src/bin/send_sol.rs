use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use dotenvy::dotenv;

use std::error::Error;
use std::str::FromStr;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let private_key_str = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env!");

    let private_vals: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let own = Keypair::from_bytes(&private_vals)?;

    // using the devnet cluster
    let c = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Own public key is: {}", own.pubkey());

    let peer = Pubkey::from_str("5y4zZNG66SQ3uCMeDo4HQ2bvZqWm1z85kjvY4RxsjS48")?; // Solana Monkey Business
                                                                                  // ( but devnet )
    println!("💸 Attempting to send 0.01 SOL to {}...", peer);

    let send_sol_instruction = system_instruction::transfer(
        &own.pubkey(),
        &peer,
        (0.01 * LAMPORTS_PER_SOL as f64) as u64,
    );

    let mut t = Transaction::new_with_payer(
        &[send_sol_instruction],
        Some(&own.pubkey()),
    );

    let recent_blockhash = c.get_latest_blockhash()?;
    t.sign(&[&own], recent_blockhash);

    let sign = c.send_and_confirm_transaction(&t)?;

    println!("✅ Transaction confirmed, signature: {}", sign);

    Ok(())
}


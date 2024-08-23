use solana_client::rpc_client::RpcClient;

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

use spl_associated_token_account::{get_associated_token_address, create_associated_token_account};
use dotenvy::dotenv;

use std::env;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let private_key_str = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env!");

    let private_vals: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let own = Keypair::from_bytes(&private_vals)?;

    let conn = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Own public key is: {}", own.pubkey());

    let token_mint = Pubkey::from_str("55TRnnGvhu2B7jKCJ2gpLFwSY8HExgJSvP4PVMq6Nw5a")?; // RustyCoin Token Mint
    let peer = Pubkey::from_str("hyEJwGwqWgqNoyrjoWaK7R2ruvPjQxpYdPVBd9SgZkw")?;

    let address = get_associated_token_address(&peer, &token_mint);

    if conn.get_account(&address).is_err() {
        let ins = create_associated_token_account(&own.pubkey(), &peer, &token_mint);

        let mut t = Transaction::new_with_payer(
            &[ins],
            Some(&own.pubkey()),
        );

        let recent_blockhash = conn.get_latest_blockhash()?;
        t.sign(&[&own], recent_blockhash);

        // Send and confirm the transaction
        let sign = conn.send_and_confirm_transaction(&t)?;

        println!("✅ Created associated token account with signature: {}", sign);
    } else {
        println!("Retrieved existing associated token account!");
    }

    println!("Token Account: {}", address);

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        address
    );
    println!("🔗 Explorer link: {}", link);

    Ok(())
}


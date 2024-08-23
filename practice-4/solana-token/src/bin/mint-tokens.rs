use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use spl_token::instruction::mint_to;
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

    // has two decimal places == 99 kopijkas is nearly a hryvnia
    let MINOR_UNITS_PER_MAJOR_UNITS = 10_u64.pow(2);

    let mint_account = Pubkey::from_str("55TRnnGvhu2B7jKCJ2gpLFwSY8HExgJSvP4PVMq6Nw5a")?; // token
                                                                                         // account
                                                                                         // for
                                                                                         // RustyCoin
    let peer_account = Pubkey::from_str("AWRRUgDfzDTjojVaTWFmvXrnW4tDABxdi2YYAkz9Qwvq")?;

    // instruction minting tokens into our guys pocket
    let mint_to_ix = mint_to(
        &spl_token::id(),
        &mint_account,
        &peer_account,
        &own.pubkey(),
        &[],
        100 * MINOR_UNITS_PER_MAJOR_UNITS,
    )?;

    let mut t = Transaction::new_with_payer(
        &[mint_to_ix],
        Some(&own.pubkey()),
    );

    let recent_blockhash = conn.get_latest_blockhash()?;
    t.sign(&[&own], recent_blockhash);

    let signed = conn.send_and_confirm_transaction(&t)?;

    let link = format!("https://explorer.solana.com/tx/{}?cluster=devnet", signed);

    println!("✅ Success! Our richman now holds 100 tokens! Transaction: {}", link);

    Ok(())
}


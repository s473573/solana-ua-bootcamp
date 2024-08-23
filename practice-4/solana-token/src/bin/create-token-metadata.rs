use solana_client::rpc_client::RpcClient;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;

use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let secret = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env!");

    let secret_vals: Vec<u8> = serde_json::from_str(&secret)?;
    let own = Keypair::from_bytes(&secret_vals)?;

    let conn = RpcClient::new("https://api.devnet.solana.com");

    let mint_account = Pubkey::from_str("55TRnnGvhu2B7jKCJ2gpLFwSY8HExgJSvP4PVMq6Nw5a")?;

    // pda derivation
    let (metadata_PDA, _metadata_bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
            &mint_account.to_bytes(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    let cma_ix = create_metadata_accounts_v3(
        TOKEN_METADATA_PROGRAM_ID,
        metadata_PDA,
        mint_account,
        own.pubkey(),
        own.pubkey(),
        own.pubkey(),
        "RustedCoin".into(), // they say it inscreases your item discovery :)
        "CRAB".into(),
        "https://spacehey.com/".into(),
        None,
        1,
        true,
        false,
        None,
        None,
        None,
    );

    let mut t = Transaction::new_with_payer(
        &[cma_ix],
        Some(&own.pubkey()),
    );

    let recent_blockhash = conn.get_latest_blockhash()?;
    t.sign(&[&own], recent_blockhash);

    let _signed = conn.send_and_confirm_transaction(&t)?;

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint_account
    );
    println!("✅ Updated token metadata. Take a look: {}!", link);

    Ok(())
}


use solana_client::rpc_client::RpcClient;

use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;
use solana_sdk::program_pack::Pack;

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction::create_account;

use dotenvy::dotenv;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let private_key_str = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env!");

    let private_vals: Vec<u8> = serde_json::from_str(&private_key_str)?;
    let own = Keypair::from_bytes(&private_vals)?;

    let conn = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Own public key is: {}", own.pubkey());

    // I am the authority!
    let mint_authority = own.pubkey();

    // mint is just a new keypair ( identity )
    let token_mint = Keypair::new();

    let mint_rent_exemption = conn.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let mut t = Transaction::new_with_payer(
        &[
            // looks simpler then typescript
            create_account(
                &own.pubkey(),
                &token_mint.pubkey(),
                mint_rent_exemption,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            initialize_mint(
                &spl_token::id(),
                &token_mint.pubkey(),
                &mint_authority,
                None,
                2, // decimal places
            )?,
        ],
        Some(&own.pubkey()),
    );

    let recent_blockhash = conn.get_latest_blockhash()?;
    t.sign(&[&own, &token_mint], recent_blockhash);

    let sign = conn.send_and_confirm_transaction(&t)?;

    println!("✅ Token Mint created successfully!");
    println!("Signature: {}", sign);

    let link = format!("https://explorer.solana.com/address/{}?cluster=devnet", token_mint.pubkey());
    println!("🔗 Token Mint Explorer Link: {}", link);

    Ok(())
}


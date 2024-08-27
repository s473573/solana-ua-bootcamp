use solana_client::rpc_client::RpcClient;
use spl_token::instruction::initialize_multisig;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction::create_account;
use solana_sdk::program_pack::Pack;

use dotenvy::dotenv;

use std::fs::File;
use std::env;
use std::str::FromStr;
use std::error::Error;
use std::io::Write;


fn main() -> Result<(), Box<dyn Error>> {
    let conn = RpcClient::new("https://api.devnet.solana.com");

    dotenv().ok();
    let secret = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env!");

    let secret_vals: Vec<u8> = serde_json::from_str(&secret)?;
    let own = Keypair::from_bytes(&secret_vals)?;

    let party_pubkey: Vec<Pubkey> = vec![
        own.pubkey(),
        Pubkey::from_str("6b8aBTbjZa8cnbtAjrzA6X5bKbywW8VAyDV79916MPTe")?, // peer_wallet
        Pubkey::from_str("ARpicXFDtV49FUP6nmLiYpovsUX8FKw4QUZgpNv75cpT")?, // multisig_wallet
    ];

    let min_signers = 2;

    // new account identity
    let multisig = Keypair::new();
    let mut file = File::create("multisig.txt")?;
    writeln!(file, "Private Key: {:?}", multisig.to_bytes())?;

    let multisig_rent_exemption = conn.get_minimum_balance_for_rent_exemption(spl_token::state::Multisig::LEN)?;

    let create_ix = create_account(
        &own.pubkey(),
        &multisig.pubkey(),
        multisig_rent_exemption,
        spl_token::state::Multisig::LEN as u64,
        &spl_token::id(),
    );

    let initialize_ix = initialize_multisig(
        &spl_token::id(),
        &multisig.pubkey(),
        &party_pubkey.iter().collect::<Vec<_>>(), // there has to be an easier way
        min_signers,
    )?;

    let mut t = Transaction::new_with_payer(
        &[create_ix, initialize_ix],
        Some(&own.pubkey()), // payer is the newly created multisig account
    );

    let recent_blockhash = conn.get_latest_blockhash()?;
    t.sign(&[&own, &multisig], recent_blockhash);

    let sign = conn.send_and_confirm_transaction(&t)?;

    println!("✅ Created a multisig authority! Transaction signature: {}", sign);
    println!("Multisig account address: {}", multisig.pubkey());

    Ok(())
}


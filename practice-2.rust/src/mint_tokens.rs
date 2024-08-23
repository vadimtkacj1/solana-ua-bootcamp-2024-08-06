use dotenv::dotenv;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use spl_token::instruction::mint_to;
use std::env;
use std::str::FromStr;

pub fn mint_tokens(token_mint: &String, token_account: &String) -> String {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    let token_mint_account = Pubkey::from_str(&token_mint)
        .expect("Invalid public key");
    let recipient_associated_token_account = Pubkey::from_str(&token_account)
        .expect("Invalid public key");

    const MINOR_UNITS_PER_MAJOR_UNITS: u64 = (10 as u64).pow(2);

    let mint_to_instruction = mint_to(
        &spl_token::id(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * MINOR_UNITS_PER_MAJOR_UNITS
    ).expect("Bad instruction");

    let blockhash = connection.get_latest_blockhash().expect("Bad connection");

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        blockhash
    );

    let signature = connection.send_and_confirm_transaction(&transaction).expect("Bad transaction");

    let link = get_explorer_link("transaction", &signature.to_string(), "devnet");

    println!("✅ Success! Mint Token Transaction: {}", link);

    return signature.to_string();
}

fn get_explorer_link(address_type: &str, address: &str, network: &str) -> String {
    format!(
        "https://explorer.solana.com/{}/{}?cluster={}",
        address_type, address, network
    )
}
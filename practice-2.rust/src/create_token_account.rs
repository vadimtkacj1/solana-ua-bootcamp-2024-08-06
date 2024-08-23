use dotenv::dotenv;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::initialize_account,
    state::Account,
};
use std::env;
use std::str::FromStr;

pub fn create_token_account(token_mint: &String) -> String {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str(&token_mint)
        .expect("Invalid public key");
    let recipient = Pubkey::from_str("GCWUHj5LqLvMH4bx3KoSG978UmNX5N78usyXMg43JcMS")
        .expect("Invalid public key");

    let token_account = Keypair::new();
    let account_rent = connection.get_minimum_balance_for_rent_exemption(Account::LEN).expect("Bad connection");

    let create_account_instruction = system_instruction::create_account(
        &sender.pubkey(), 
        &token_account.pubkey(),
        account_rent,
        Account::LEN as u64,
        &spl_token::id()
    );

    let initialize_account_instruction = initialize_account(
        &spl_token::id(), 
        &token_account.pubkey(), 
        &token_mint_account, 
        &recipient
    ).expect("Bad instruction");

    let blockhash = connection.get_latest_blockhash().expect("Bad connection");

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_account_instruction],
        Some(&sender.pubkey()),
        &[&sender, &token_account],
        blockhash
    );

    connection.send_and_confirm_transaction(&transaction).expect("Bad transaction");

    println!("Token Acount: {}", token_account.pubkey());

    let link = get_explorer_link("address", &token_account.pubkey().to_string(), "devnet");

    println!("✅ Created token account: {}", link);

    return token_account.pubkey().to_string();
}

fn get_explorer_link(address_type: &str, address: &str, network: &str) -> String {
    format!(
        "https://explorer.solana.com/{}/{}?cluster={}",
        address_type, address, network
    )
}
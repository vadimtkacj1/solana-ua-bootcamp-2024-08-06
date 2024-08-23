use dotenv::dotenv;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    program_pack::Pack,
};
use spl_token::{
    instruction::initialize_mint,
    state::Mint,
};
use std::env;

pub fn create_token_mint() -> String{
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Our public key is: {}", sender.pubkey());

    let mint = Keypair::new();
    let mint_rent = connection.get_minimum_balance_for_rent_exemption(Mint::LEN).expect("Bad connection");
    
    let create_account_instruction = system_instruction::create_account(
        &sender.pubkey(), 
        &mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::id()
    );

    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(), 
        &mint.pubkey(), 
        &sender.pubkey(), 
        None, 
        2
    ).expect("Bad instruction");

    let blockhash = connection.get_latest_blockhash().expect("Bad connection");

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&sender.pubkey()),
        &[&sender, &mint],
        blockhash
    );

    connection.send_and_confirm_transaction(&transaction).expect("Bad transaction");

    let link = get_explorer_link("address", &mint.pubkey().to_string(), "devnet");

    println!("✅ Token Mint: {}", link);

    return mint.pubkey().to_string();
}

fn get_explorer_link(address_type: &str, address: &str, network: &str) -> String {
    format!(
        "https://explorer.solana.com/{}/{}?cluster={}",
        address_type, address, network
    )
}
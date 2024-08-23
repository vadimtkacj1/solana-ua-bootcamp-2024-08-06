use dotenv::dotenv;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::env;
use std::str::FromStr;

pub fn send_sol() {
    dotenv().ok();
    
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("GCWUHj5LqLvMH4bx3KoSG978UmNX5N78usyXMg43JcMS")
        .expect("Invalid public key");
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    let send_sol_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient,
        (0.01 * LAMPORTS_PER_SOL as f64) as u64,
    );

    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
        .expect("Invalid public key");

    let memo_text = "Hello from Solana!";

    let add_memo_instruction = Instruction {
        accounts: vec![AccountMeta {
            pubkey: sender.pubkey(),
            is_signer: true,
            is_writable: true,
        }],
        data: memo_text.as_bytes().to_vec(),
        program_id: memo_program,
    };

    println!("📝 memo is: {}", memo_text);

    let blockhash = connection.get_latest_blockhash().expect("Bad connection");

    let transaction = Transaction::new_signed_with_payer(
        &[send_sol_instruction, add_memo_instruction],
        Some(&sender.pubkey()),
        &[&sender],
        blockhash
    );

    let signature = connection.send_and_confirm_transaction(&transaction).expect("Bad transaction");

    println!("✅ Transaction confirmed, signature: {}!", signature);
}
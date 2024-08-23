use dotenv::dotenv;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use mpl_token_metadata::{
    types::DataV2,
    instructions::{
        CreateMetadataAccountV3,
        CreateMetadataAccountV3InstructionArgs,
    }
};
use std::env;
use std::str::FromStr;

pub fn create_token_metadata() {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let user = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    let token_metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
        .expect("Invalid public key");
    let token_mint_account = Pubkey::from_str("8c6EPJeX1iQYyVrp6KQ5RPiEmq2r2fbao7R9MzM5CQwi")
        .expect("Invalid public key");

    let rent_pubkey = Pubkey::from_str("13sadsdqwdacs")
        .expect("Invalid piblic key");

    let metadata_data = DataV2 {
        name: String::from("Solana UA Bootcamp 2024-08-06"),
        symbol: String::from("UAB-2"),
        uri: String::from("https://arweave.net/1234"),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let (metadata_pda, _metadata_bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            &token_metadata_program_id.to_bytes(),
            &token_mint_account.to_bytes(),
        ],
        &token_metadata_program_id
    );

    let create_instruction = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: token_mint_account,
        mint_authority: user.pubkey(),
        payer: user.pubkey(),
        update_authority: (user.pubkey(), true),
        system_program: token_metadata_program_id,
        rent: Some(rent_pubkey),
    };

    let create_instruction_args = CreateMetadataAccountV3InstructionArgs {
        collection_details: None,
        data: metadata_data,
        is_mutable: true,
    };

    let metadata_instruction = create_instruction.instruction(create_instruction_args);

    let blockhash = connection.get_latest_blockhash().expect("Bad connection");
    
    let transaction = Transaction::new_signed_with_payer(
        &[metadata_instruction],
        Some(&user.pubkey()),
        &[&user],
        blockhash
    );

    connection.send_and_confirm_transaction(&transaction).expect("Bad transaction");

    let link = get_explorer_link("address", &token_mint_account.to_string(), "devnet");

    println!("✅ Look at the token mint again: {} !", link);
}

fn get_explorer_link(address_type: &str, address: &str, network: &str) -> String {
    format!(
        "https://explorer.solana.com/{}/{}?cluster={}",
        address_type, address, network
    )
}
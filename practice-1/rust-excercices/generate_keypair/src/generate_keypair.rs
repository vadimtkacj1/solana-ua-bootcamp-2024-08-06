use solana_sdk::signature::{Keypair, Signer};

pub fn generate_keypair() {
        let keypair = Keypair::new();

            println!("The public key is: {}", keypair.pubkey().to_string());
                println!("The secret key is: {:?}", keypair.to_bytes());
                    println!("✅ Finished!");
}

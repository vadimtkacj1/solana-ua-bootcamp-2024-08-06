use solana_sdk::signature::{Keypair, Signer};
use dotenv::dotenv;
use std::env;
use std::process;
use serde_json;

pub fn load_keypair() {
        dotenv().ok();

            let private_key = match env::var("SECRET_KEY") {
                        Ok(val) => val,
                                Err(_) => {
                                                println!("Add SECRET_KEY to .env!");
                                                            process::exit(1);
                                                                    }
                            };

                let as_array: Vec<u8> = match serde_json::from_str(&private_key) {
                            Ok(arr) => arr,
                                    Err(_) => {
                                                    println!("Invalid SECRET_KEY format!");
                                                                process::exit(1);
                                                                        }
                                };

                    let keypair = Keypair::from_bytes(&as_array).expect("Invalid secret key length");

                        println!("Public key: {}", keypair.pubkey().to_string());
}

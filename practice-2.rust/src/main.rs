mod send_sol;
mod create_token_mint;
mod create_token_account;
mod mint_tokens;
mod create_token_metadata;

fn main() {
    send_sol::send_sol();
    println!();
    let token_mint = create_token_mint::create_token_mint();
    println!();
    let token_account = create_token_account::create_token_account(&token_mint);
    println!();
    let mint_tokens = mint_tokens::mint_tokens(&token_mint, &token_account);
    println!();

}
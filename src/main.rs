use tokio;
use std::time::{SystemTime};
use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{ signature::{Keypair, Signature, Signer},
transaction::VersionedTransaction,
message::v0::Message};
use quote_fetcher::get_quote;
use swap_instructions::swap_instructions;
use sign_txn::sign_txn;

pub mod sign_txn;
pub mod swap_instructions;
pub mod quote_fetcher;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    
    println!("keypair.pubkey(): {}", keypair.pubkey());
    let usdc_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let sol_mint = "So11111111111111111111111111111111111111112";
    let amount = 10000000; 
    println!("Time: {:?}", SystemTime::now());
    let (quote1amt, quote1route, quote1other_amount_threshold, quote1amm_key, quote1fee_amount) = get_quote(usdc_mint.to_string(), sol_mint.to_string(), amount).await.unwrap();
    let (quote2amt, quote2route, _quote2other_amount_threshold, _quote2amm_key, _quote2fee_amount) = get_quote(sol_mint.to_string(), usdc_mint.to_string(), quote1amt.parse::<u64>().unwrap()).await.unwrap();
  
    let profit_percent = (quote2amt.parse::<u64>().unwrap() as f64 / amount as f64 - 1.0) * 100.0;
    if quote2amt.parse::<u64>().unwrap() > amount {
        println!("Arbitrage opportunity found! Buy {} SOL for {} USDC on route {}", quote1amt, amount, quote1route);
        println!("Sell {} USDC for {} SOL on route {}", quote2amt, amount, quote2route);
        println!("Profit % = {}", profit_percent);
    } else {
        println!("No arbitrage opportunity found.");
    }
    
    print!("Time: {:?}", SystemTime::now());

    // Call the swap_instructions function with the necessary parameters
    let swap_txn = swap_instructions(
        usdc_mint.to_string(),
        sol_mint.to_string(),
        amount.to_string(),
        quote1amt,
        quote1other_amount_threshold,
        quote1route,
        quote1amm_key,
        quote1fee_amount,
    ).await.expect("Failed to create swap instructions");

    sign_txn(swap_txn).await.expect("Failed to sign transaction");

    println!("Time: {:?}", SystemTime::now());

}

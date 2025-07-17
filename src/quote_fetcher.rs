use reqwest::{Client, header::HeaderMap};
// use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};


pub async fn get_quote(input_mint: String, output_mint: String, amount:u64) -> Result<(String, String, String, String, String)> {

    let url = format!("https://lite-api.jup.ag/swap/v1/quote?inputMint={}&outputMint={}&amount={}", input_mint, output_mint, amount);

    let client = Client::builder()
    .build()
    .context("Failed to build HTTP client")?;
      


    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());

    let request = client.request(reqwest::Method::GET, url)
        .headers(headers);

     let response = request.send().await
    .context("Failed to send request")?;

    let body: serde_json::Value = response
    .json()
    .await
    .context("Failed to parse response as JSON")?;
    // println!("Body: {}", body);
    // println!("Best route: {}", body["routePlan"][0]["swapInfo"]["label"]);
    // println!("Output amount: {}", body["routePlan"][0]["swapInfo"]["outAmount"]);
    Ok((body["routePlan"][0]["swapInfo"]["outAmount"].as_str().unwrap().to_string(), 
    body["routePlan"][0]["swapInfo"]["label"].as_str().unwrap().to_string(), 
    body["otherAmountThreshold"].as_str().unwrap().to_string(),
    body["routePlan"][0]["swapInfo"]["ammKey"].as_str().unwrap().to_string(),
    body["routePlan"][0]["swapInfo"]["feeAmount"].as_str().unwrap().to_string()))
}
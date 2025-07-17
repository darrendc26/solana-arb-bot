use reqwest::{Client, header::HeaderMap};
use serde_json::json;
use anyhow::{Result, Context};
use std::env;
use dotenv::dotenv;

pub async fn swap_instructions(
    input_mint: String,
    output_mint: String,
    in_amount: String,
    out_amount: String,
    other_amount_threshold: String,
    label: String,
    amm_key: String,
    fee_amount: String,
) -> Result<String> {
    dotenv().ok();
    let user_public_key = env::var("USER_PUBLIC_KEY")
        .context("USER_PUBLIC_KEY not set in .env file")?;

    let url = "https://lite-api.jup.ag/swap/v1/swap-instructions";

    let client = Client::builder()
        .build()
        .context("Failed to build HTTP client")?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());

    let data = json!({
        "userPublicKey": user_public_key,
        "quoteResponse": {
            "inputMint": input_mint,
            "inAmount": in_amount,
            "outputMint": output_mint,
            "outAmount": out_amount,
            "otherAmountThreshold": other_amount_threshold,
            "swapMode": "ExactIn",
            "slippageBps": 50,
            "platformFee": null,
            "priceImpactPct": "0",
            "routePlan": [
                {
                    "swapInfo": {
                        "ammKey": amm_key,
                        "label": label,
                        "inputMint": input_mint,
                        "outputMint": output_mint,
                        "inAmount": in_amount,
                        "outAmount": out_amount,
                        "feeAmount": fee_amount,
                        "feeMint": output_mint
                    },
                    "percent": 100
                }
            ]
        },
        "prioritizationFeeLamports": {
            "priorityLevelWithMaxLamports": {
                "maxLamports": 10000000,
                "priorityLevel": "veryHigh"
            }
        },
        "dynamicComputeUnitLimit": true
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .context("Failed to send request")?;

    let body = response.text().await.context("Failed to read response")?;
    println!("{}", body);

    Ok(body)
}

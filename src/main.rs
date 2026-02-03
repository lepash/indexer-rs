use serde_json::json;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("INFURA_API_KEY")
        .expect("INFURA_API_KEY must be set in .env file");

    let client = reqwest::Client::new();
    let body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": ["latest", true],
            "id": 1
    });

    let url = format!("https://mainnet.infura.io/v3/{}", api_key);
    let resp = client.post(url)
        .json(&body)
        .send()
        .await?;
    
    let resp_json: serde_json::Value = resp.json().await?;
    let transaction = &resp_json["result"]["transactions"].as_array().unwrap()[0];
    // println!("Number of transactions in the latest block: {}", transactions.len());
    println!("{:?}", transaction.as_object().unwrap());
    
    Ok(())
}
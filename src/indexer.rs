use serde_json::json;

use crate::db;

pub struct Indexer<'a> {
    api_key: String,
    database_url: &'a str,
    db: db::Db,
    client: reqwest::Client,
}

impl<'a> Indexer<'a> {
    pub async fn new(api_key: String, database_url: &'a str) -> Self {
        Self { 
            api_key, 
            database_url, 
            db: db::Db::new(database_url).await, 
            client: reqwest::Client::new() 
        }
    }

    async fn run() {}

    async fn is_eoa(&self, address: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getCode",
            "params": [address, "latest"],
            "id": 1
        });

        let resp_json = self.rpc_call(&body).await?;
        let code = resp_json["result"].as_str().unwrap_or("");
        Ok(code == "0x")
    }

    async fn rpc_call(&self, body: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("https://mainnet.infura.io/v3/{}", self.api_key);
        let resp = self.client.post(url)
            .json(&body)
            .send()
            .await?;

        let resp_json: serde_json::Value = resp.json().await?;
        Ok(resp_json)
    }

    async fn fetch_new_block(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": ["latest", true],
            "id": 1
        });

        let resp_json = self.rpc_call(&body).await?;
        let transaction = &resp_json["result"]["transactions"].as_array().unwrap()[0];
        println!("Latest block: {:?}", resp_json);
        Ok(())
    }
}







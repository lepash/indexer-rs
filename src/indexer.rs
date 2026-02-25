use anyhow::{Ok, Result};
use chrono::DateTime;
use serde_json::json;
use crate::db;
use crate::utils;

pub struct Indexer<'a> {
    api_key: String,
    database_url: &'a str,
    pub db: db::Db,
    client: reqwest::Client,
    poll_interval: u64,
}

impl<'a> Indexer<'a> {
    pub async fn new(api_key: String, database_url: &'a str, poll_interval: u64) -> Self {
        Self { 
            api_key, 
            database_url, 
            db: db::Db::new(database_url).await, 
            client: reqwest::Client::new(),
            poll_interval,
        }
    }

    pub async fn run(&self) {
        // loop {
            let block = self.fetch_new_block().await.unwrap();
            let transfers = self.extract_transfers(&block).await.unwrap();

        //     tokio::time::sleep(tokio::time::Duration::from_millis(self.poll_interval)).await;
        // }
    }

    async fn is_eoa(&self, address: &str) -> anyhow::Result<bool> {
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

    async fn rpc_call(&self, body: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
        let url = format!("https://mainnet.infura.io/v3/{}", self.api_key);
        let resp = self.client.post(url)
            .json(&body)
            .send()
            .await?;

        let resp_json: serde_json::Value = resp.json().await?;
        Ok(resp_json)
    }

    pub async fn fetch_new_block(&self) -> anyhow::Result<serde_json::Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": ["latest", true],
            "id": 1
        });

        let resp_json = self.rpc_call(&body).await?;
        Ok(resp_json)
    }

    pub async fn extract_transfers(&self, block: &serde_json::Value) -> anyhow::Result<()> {
        let timestamp = block["result"]["timestamp"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No timestamp found in block data"))?;
        let unix_timestamp = utils::parse_hex(timestamp)?;
        let dt = DateTime::from_timestamp(unix_timestamp, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse timestamp"))?;
        println!("Block timestamp: {}", dt);

        let transactions = block["result"]["transactions"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No transactions found in block data"))?;
    
        for tx in transactions {
            let to_address = tx["to"].as_str().unwrap_or("");
            let from_address = tx["from"].as_str().unwrap_or("");

            if self.is_eoa(to_address).await? && self.is_eoa(from_address).await? {
                print!("{:?}", tx);
                self.parse_transaction(&tx, dt).await?;
            }

        }
        
        Ok(())
    }

    async fn parse_transaction(&self, transaction: &serde_json::Value, timestamp: chrono::DateTime<chrono::Utc>) -> anyhow::Result<()> {
        let tx_hash = transaction["hash"].as_str().unwrap_or("");
        let block_number = utils::parse_hex(transaction["blockNumber"].as_str().unwrap_or("0"))?;
        let txfrom = transaction["from"].as_str().unwrap_or("");
        let txto = transaction["to"].as_str().unwrap_or("");
        let value = utils::parse_hex(transaction["value"].as_str().unwrap_or("0"))?;
        let gas_price = utils::parse_hex(transaction["gasPrice"].as_str().unwrap_or("0"))?;
        let gas = utils::parse_hex(transaction["gas"].as_str().unwrap_or("0"))?;
        let maxFeePerGas = utils::parse_hex(transaction["maxFeePerGas"].as_str().unwrap_or("0"))?;
        let maxPriorityFeePerGas = utils::parse_hex(transaction["maxPriorityFeePerGas"].as_str().unwrap_or("0"))?;


        println!("Parsed transaction: hash={}, from={}, to={}, value={}, gas_price={}, gas={}, maxFeePerGas={}, maxPriorityFeePerGas={}", tx_hash, txfrom, txto, value, gas_price, gas, maxFeePerGas, maxPriorityFeePerGas);
        self.db.insert_transfer(
            tx_hash, 
            block_number, 
            timestamp, 
            txfrom, 
            txto,
            value,
            gas_price, 
            gas,
            maxFeePerGas,
            maxPriorityFeePerGas
        )
            .await?;

        Ok(())
    }

}





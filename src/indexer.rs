use bigdecimal::BigDecimal;
use chrono::DateTime;
use serde_json::json;
use crate::db;
use crate::utils;

pub struct Indexer {
    api_key: String,
    pub db: db::Db,
    client: reqwest::Client,
    poll_interval: u64,
}

impl Indexer {
    pub async fn new(api_key: String, database_url: String, poll_interval: u64) -> anyhow::Result<Self> {
        Ok(
            Self { 
            api_key, 
            db: db::Db::new(&database_url).await?, 
            client: reqwest::Client::new(),
            poll_interval,
        }
    )
    }

    pub async fn run(&self) {
        loop {

            if let Some(max_block_indexed) = self.db.get_max_block().await.unwrap() {
                let latest_block = self.fetch_block("latest").await.unwrap();
                let latest_block_number = utils::parse_hex_i64(latest_block["result"]["number"].as_str().unwrap_or("0"));
                if latest_block_number.as_ref().is_err() {
                    println!("Failed to parse latest block number: {:?}", latest_block_number.err());
                    continue;
                }

                if *latest_block_number.as_ref().unwrap() <= max_block_indexed {
                    println!("No new blocks to index. Latest block: {}, Max block indexed: {}", latest_block_number.unwrap(), max_block_indexed);
                    continue;
                }

                for block_number in (max_block_indexed + 1)..=latest_block_number.unwrap() {
                    let block = self.fetch_block(&format!("0x{:x}", block_number)).await.unwrap();
                    self.extract_transfers(&block).await.unwrap();
                    println!("Indexed block number: {}", block_number);
                }

            } else {
                println!("No blocks indexed yet. Cold start from the latest block.");
                let latest_block = self.fetch_block("latest").await.unwrap();
                let block_number = self.extract_transfers(&latest_block).await.unwrap();
                println!("Indexed block number: {}", block_number);
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(self.poll_interval)).await;
        }
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

    pub async fn fetch_block(&self, param: &str) -> anyhow::Result<serde_json::Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [param, true],
            "id": 1
        });

        let resp_json = self.rpc_call(&body).await?;
        Ok(resp_json)
    }

    pub async fn extract_transfers(&self, block: &serde_json::Value) -> anyhow::Result<BigDecimal> {
        let timestamp = block["result"]["timestamp"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No timestamp found in block data"))?;
        let unix_timestamp = utils::parse_hex_i64(timestamp)?;
        let dt = DateTime::from_timestamp(unix_timestamp, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse timestamp"))?;
        println!("Block timestamp: {}", dt);

        let transactions = block["result"]["transactions"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("No transactions found in block data"))?;
        
        let block_number = utils::parse_hex(block["result"]["number"].as_str().unwrap_or("0"))?;
        
        for tx in transactions {
            let to_address = tx["to"].as_str().unwrap_or("");
            let from_address = tx["from"].as_str().unwrap_or("");

            if self.is_eoa(to_address).await? && self.is_eoa(from_address).await? {
                // print!("{:?}", tx);
                self.parse_transaction(&tx, dt).await?;
            }
        }
        
        Ok(block_number)
    }

    async fn parse_transaction(&self, transaction: &serde_json::Value, timestamp: chrono::DateTime<chrono::Utc>) -> anyhow::Result<()> {
        let tx_hash = transaction["hash"].as_str().unwrap_or("");
        let block_number = utils::parse_hex(transaction["blockNumber"].as_str().unwrap_or("0"))?;
        let txfrom = transaction["from"].as_str().unwrap_or("");
        let txto = transaction["to"].as_str().unwrap_or("");
        let value = utils::parse_hex(transaction["value"].as_str().unwrap_or("0"))?;
        let gas_price = utils::parse_hex(transaction["gasPrice"].as_str().unwrap_or("0"))?;
        let gas = utils::parse_hex(transaction["gas"].as_str().unwrap_or("0"))?;
        let max_fee_per_gas = utils::parse_hex(transaction["maxFeePerGas"].as_str().unwrap_or("0"))?;
        let max_priority_fee_per_gas = utils::parse_hex(transaction["maxPriorityFeePerGas"].as_str().unwrap_or("0"))?;


        println!("Parsed transaction: hash={}, from={}, to={}, value={}, gas_price={}, gas={}, maxFeePerGas={}, maxPriorityFeePerGas={}", tx_hash, txfrom, txto, value, gas_price, gas, max_fee_per_gas, max_priority_fee_per_gas);
        self.db.insert_transfer(
            tx_hash, 
            block_number, 
            timestamp, 
            txfrom, 
            txto,
            value,
            gas_price, 
            gas,
            max_fee_per_gas,
            max_priority_fee_per_gas
        )
            .await?;

        Ok(())
    }

}





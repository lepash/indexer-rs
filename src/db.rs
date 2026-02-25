use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, Debug)] 
pub struct NativeCurrencyTransfer { 
    pub tx_hash: String, 
    pub block_number: i64, 
    pub timestamp: chrono::DateTime<chrono::Utc>, 
    pub txfrom: String, 
    pub txto: String, 
    pub value: i64, 
    pub gas_price: i64,
    pub gas: i64,
    pub max_fee_per_gas: i64,
    pub max_priority_fee_per_gas: i64,
}

pub struct Db { 
    pub pool: sqlx::PgPool, 
}

impl Db {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5) 
            .connect(database_url)
            .await 
            .expect("Failed to create database pool");
        
        sqlx::query(
        "CREATE TABLE IF NOT EXISTS native_currency_transfers ( 
                tx_hash TEXT NOT NULL PRIMARY KEY, 
                block_number BIGINT NOT NULL, 
                timestamp TIMESTAMPTZ NOT NULL, 
                txfrom TEXT NOT NULL, 
                txto TEXT NOT NULL, 
                value INT8 NOT NULL,
                gas_price INT8,
                gas INT8,
                max_fee_per_gas INT8,
                max_priority_fee_per_gas INT8
        )")
        .execute(&pool) 
        .await?;

        Ok(Self { pool })
    }

    async fn get_max_block(&self) -> anyhow::Result<Option<i64>> { 
        let (max_block,): (Option<i64>,) = sqlx::query_as("SELECT MAX(block_number) as max_block FROM native_currency_transfers") /* returns a row that contains a single column. (_,) represents a row with a single column */
            .fetch_one(&self.pool)
            .await?;

        Ok(max_block)
     }
    
    /*Note for myself in the future: i64 wastes one bit for the sign since block numbers, gas, fees, etc can't be negative but sqlx doesn't natively support it */
    pub async fn insert_transfer(&self, tx_hash: &str, block_number: i64, timestamp: chrono::DateTime<chrono::Utc>, txfrom: &str, txto: &str, value: i64, gas_price: i64, gas: i64, max_fee_per_gas: i64, max_priority_fee_per_gas: i64) -> anyhow::Result<()> { 
        sqlx::query( "INSERT INTO native_currency_transfers (tx_hash, block_number, timestamp, txfrom, txto, value, gas_price, gas, max_fee_per_gas, max_priority_fee_per_gas) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (tx_hash) DO NOTHING" ) 
            .bind(tx_hash) 
            .bind(block_number) 
            .bind(timestamp) 
            .bind(txfrom) 
            .bind(txto) 
            .bind(value) 
            .bind(gas_price) 
            .bind(gas)  
            .bind(max_fee_per_gas)
            .bind(max_priority_fee_per_gas)
            .execute(&self.pool) 
            .await?; 

        Ok(())
    }

    pub async fn select_all_transfers(&self) -> anyhow::Result<Vec<NativeCurrencyTransfer>> { 
        let transfers = sqlx::query_as::<_, NativeCurrencyTransfer>("SELECT * FROM native_currency_transfers") 
            .fetch_all(&self.pool) 
            .await?;

        Ok(transfers)
    }

    pub async fn read_transfers_from_address(&self, txfrom: &str) -> anyhow::Result<Vec<NativeCurrencyTransfer>> { 
        let transfer  = sqlx::query_as::<_, NativeCurrencyTransfer>( "SELECT * FROM native_currency_transfers WHERE txfrom = $1" ) 
            .bind(txfrom) 
            .fetch_all(&self.pool) 
            .await?; 

        Ok(transfer)
    }

    pub async fn read_transfer_to_address(&self, txto: &str) -> anyhow::Result<Vec<NativeCurrencyTransfer>> { 
        let transfer  = sqlx::query_as::<_, NativeCurrencyTransfer>( "SELECT * FROM native_currency_transfers WHERE txto = $1" ) 
            .bind(txto) 
            .fetch_all(&self.pool) 
            .await?; 

        Ok(transfer)
    } 

    pub async fn reset_db(&self) -> anyhow::Result<()> { 
        sqlx::query("DROP TABLE IF EXISTS native_currency_transfers") 
            .execute(&self.pool) 
            .await?; 

        Ok(())
    }
}


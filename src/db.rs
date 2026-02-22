use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow)] 
pub struct NativeCurrencyTransfer { 
    pub transaction_hash: String, 
    pub block_number: i64, 
    pub timestamp: chrono::DateTime<chrono::Utc>, 
    pub txfrom: String, 
    pub txto: String, 
    pub value: String, 
}

pub struct Db { 
    pub pool: sqlx::PgPool, 
}

impl Db {
    pub async fn new(database_url: &str) -> Self {
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
                value NUMERIC NOT NULL,
                gas_price NUMERIC,
                gas NUMERIC
        )")
        .execute(&pool) 
        .await
        .expect("Failed to create table");

        Self { pool }
    }

    async fn get_max_block(&self) -> Option<i64> { 
        let (max_block,): (Option<i64>,) = sqlx::query_as("SELECT MAX(block_number) as max_block FROM native_currency_transfers") /* returns a row that contains a single column. (_,) represents a row with a single column */
            .fetch_one(&self.pool)
            .await
            .expect("Failed to fetch max block");

        return max_block;
     }
    
    pub async fn insert_transfer(&self, tx_hash: &str, block_number: i64, timestamp: chrono::DateTime<chrono::Utc>, txfrom: &str, txto: &str, value: &str, gas_price: &str, gas: &str) -> Result<(), sqlx::Error> { 
        sqlx::query( "INSERT INTO native_currency_transfers (tx_hash, block_number, timestamp, txfrom, txto, value, gas_price, gas) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (tx_hash) DO NOTHING" ) 
            .bind(tx_hash) 
            .bind(block_number) 
            .bind(timestamp) 
            .bind(txfrom) 
            .bind(txto) 
            .bind(value) 
            .bind(gas_price) 
            .bind(gas)  
            .execute(&self.pool) 
            .await?; 

        Ok(())
    }

    pub async fn read_transfers_from_address(&self, txfrom: &str) -> Result<Vec<NativeCurrencyTransfer>, sqlx::Error> { 
        let transfer  = sqlx::query_as::<_, NativeCurrencyTransfer>( "SELECT * FROM native_currency_transfers WHERE txfrom = $1" ) 
            .bind(txfrom) 
            .fetch_all(&self.pool) 
            .await?; 

        Ok(transfer)
    }

    pub async fn read_transfer_to_address(&self, txto: &str) -> Result<Vec<NativeCurrencyTransfer>, sqlx::Error> { 
        let transfer  = sqlx::query_as::<_, NativeCurrencyTransfer>( "SELECT * FROM native_currency_transfers WHERE txto = $1" ) 
            .bind(txto) 
            .fetch_all(&self.pool) 
            .await?; 

        Ok(transfer)
    } 

}


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
            .connect_lazy(database_url) 
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


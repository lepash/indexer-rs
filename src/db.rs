use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow)] 
pub struct NativeCurrencyTransfer { 
    pub transaction_hash: String, 
    pub block_number: i64, 
    pub timestamp: chrono::DateTime<chrono::Utc>, 
    pub from_address: String, 
    pub to_address: String, 
    pub value: String, 
}

async fn create_db(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5) 
        .connect(database_url) 
        .await?;
    
    sqlx::query(
    "CREATE TABLE IF NOT EXISTS native_currency_transfers ( 
            transaction_hash TEXT NOT NULL PRIMARY KEY, 
            block_number BIGINT NOT NULL, 
            timestamp TIMESTAMPTZ NOT NULL, 
            from_address TEXT NOT NULL, 
            to_address TEXT NOT NULL, 
            value NUMERIC NOT NULL, transaction_hash TEXT NOT NULL,
    )").execute(&pool) 
    .await?;

    Ok(())
}

async fn insert_transfer(pool: &sqlx::PgPool, transaction_hash: &str, block_number: i64, timestamp: chrono::DateTime<chrono::Utc>, from_address: &str, to_address: &str, value: &str) -> Result<(), sqlx::Error> { 
    sqlx::query( "INSERT INTO native_currency_transfers (transaction_hash, block_number, timestamp, from_address, to_address, value) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (transaction_hash) DO NOTHING" ) 
        .bind(transaction_hash) 
        .bind(block_number) 
        .bind(timestamp) 
        .bind(from_address) 
        .bind(to_address) 
        .bind(value) 
        .execute(pool) 
        .await?; 

    Ok(())
}


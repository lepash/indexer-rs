use std::env;
use dotenv::dotenv;
use indexer_rs::indexer::Indexer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); 
    env_logger::init();
    
    let api_key = env::var("INFURA_API_KEY")
        .expect("INFURA_API_KEY must be set in .env file");
    let database_url = env::var("DATABASE_URL") 
        .expect("DATABASE_URL must be set in .env file"); 
    let poll_interval = env::var("POLL_INTERVAL")
        .unwrap_or_else(|_| "5000".to_string()).parse::<u64>().expect("POLL_INTERVAL must be a valid integer");
    
    let indexer_rs = Indexer::new(api_key, &database_url, poll_interval).await;

    // indexer_rs.run().await;
    let transfers = indexer_rs.db.select_all_transfers().await?;
    println!("Found {:?} transfers", transfers);

    Ok(())
}
use std::env;
use dotenv::dotenv;
use indexer_rs::indexer::Indexer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); 

    let api_key = env::var("INFURA_API_KEY")
        .expect("INFURA_API_KEY must be set in .env file");
    let database_url = env::var("DATABASE_URL") 
        .expect("DATABASE_URL must be set in .env file"); 
    
    let indexer_rs = Indexer::new(api_key, &database_url).await;

    Ok(())
}
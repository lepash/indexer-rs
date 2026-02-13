pub struct Indexer {
    api_key: String,
    database_url: String,
}

impl Indexer {
    pub fn new(api_key: String, database_url: String) -> Self {
        Self { api_key, database_url }
    }

    async fn run() {}

    async fn is_eoa() {}

    async fn rpc_call() {}

    async fn fetch_new_block() {}
}







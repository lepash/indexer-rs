# indexer-rs

Ethereum blockchain indexer written in Rust. Fetches block and transaction data from Ethereum mainnet via Infura's JSON-RPC API.

## How it works

Uses `reqwest` to make JSON-RPC calls to Infura (`eth_getBlockByNumber`), retrieves the latest block with full transaction objects, and prints transaction data. Async runtime powered by `tokio`.

## Setup

1. Create a `.env` file:
   ```
   INFURA_API_KEY=your_key_here
   ```

2. Set up PostgreSQL:
   ```sql
   CREATE DATABASE <your_db>;
   CREATE USER <your_user> WITH PASSWORD '<your_password>';
   GRANT ALL PRIVILEGES ON DATABASE <your_db> TO <your_user>;
   ```

   Add the connection string to `.env`:
   ```
   DATABASE_URL=postgres://<your_user>:<your_password>@localhost:5432/<your_db>
   ```

3. Build and run:
   ```sh
   cargo run
   ```

## Dependencies

- **tokio** - async runtime
- **reqwest** - HTTP client
- **serde_json** - JSON parsing
- **tungstenite** - WebSocket client
- **dotenv** - env var loading

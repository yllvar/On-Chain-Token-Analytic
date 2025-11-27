# On-Chain Token Analytics API

A GraphQL API for fetching and analyzing on-chain token data from Ethereum and Solana blockchains.

## Features

- Fetch token information (name, symbol, decimals, total supply)
- Get token analytics (holders, transfers, liquidity)
- Caching layer with Redis for improved performance
- Support for both Ethereum and Solana blockchains
- Real-time data updates

## Prerequisites

- Rust (latest stable version)
- Redis server (for caching)
- Node.js (for running the GraphQL playground)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/on-chain-token-analytics.git
   cd on-chain-token-analytics
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. Start Redis server:
   ```bash
   # On macOS (using Homebrew)
   brew services start redis
   
   # Or using Docker
   docker run --name redis -p 6379:6379 -d redis
   ```

## Running the Server

```bash
# Development mode with hot-reload
cargo watch -x run

# Production mode
cargo run --release
```

The GraphQL API will be available at `http://localhost:4000/graphql`

## Example Queries

### Get Token Information

```graphql
query GetTokenInfo {
  token(address: "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984", chain: ETHEREUM) {
    name
    symbol
    decimals
    totalSupply
    priceUsd
    priceChange24h
  }
}
```

### Get Token Analytics

```graphql
query GetTokenAnalytics {
  tokenAnalytics(address: "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984", chain: ETHEREUM) {
    token {
      name
      symbol
    }
    holders(first: 5) {
      address
      balance
      valueUsd
      share
    }
    transfers(first: 5) {
      txHash
      from
      to
      amount
      timestamp
    }
  }
}
```

## Project Structure

```
src/
├── main.rs             # Application entry point
├── config.rs          # Configuration management
├── cache.rs           # Redis caching layer
├── models/            # Data models and GraphQL types
├── rpc/               # Blockchain RPC clients
│   ├── ethereum.rs    # Ethereum client implementation
│   └── solana.rs      # Solana client implementation
└── graphql/           # GraphQL schema and resolvers
    ├── schema.rs      # GraphQL schema definition
    ├── resolvers/     # Resolver implementations
    │   ├── ethereum.rs
    │   └── solana.rs
    └── cache.rs       # GraphQL-specific caching
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_HOST` | Server host | `0.0.0.0` |
| `SERVER_PORT` | Server port | `4000` |
| `REDIS_URL` | Redis connection URL | `redis://127.0.0.1:6379` |
| `ETH_RPC_URL` | Ethereum RPC endpoint | Required |
| `SOL_RPC_URL` | Solana RPC endpoint | `https://api.mainnet-beta.solana.com` |
| `CACHE_TTL_SECONDS` | Cache TTL in seconds | `300` |

## Testing

```bash
# Run tests
cargo test

# Run with logs
RUST_LOG=debug cargo test -- --nocapture
```

## License

MIT

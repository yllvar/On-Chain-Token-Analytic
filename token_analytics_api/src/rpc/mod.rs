// In src/rpc/mod.rs
pub mod ethereum;
pub mod solana;

// Re-export the client types for external use
pub use ethereum::EthereumClient;
pub use solana::SolanaClient;
use std::fmt;
use async_graphql::{SimpleObject, Enum};
use serde::{Deserialize, Serialize};

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chain::Ethereum => write!(f, "ethereum"),
            Chain::Solana => write!(f, "solana"),
            Chain::Bsc => write!(f, "bsc"),
            Chain::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Token {
    pub address: String,
    pub chain: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u8>,
    pub total_supply: Option<String>,
    pub price_usd: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Holder {
    pub address: String,
    pub balance: String,
    pub value_usd: Option<f64>,
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Transfer {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub timestamp: i64,
    pub block_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct TokenAnalytics {
    pub token: Token,
    pub holders: Vec<Holder>,
    pub transfers: Vec<Transfer>,
    pub liquidity_pools: Vec<LiquidityPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct LiquidityPool {
    pub address: String,
    pub exchange: String,
    pub token0: String,
    pub token1: String,
    pub reserve0: String,
    pub reserve1: String,
    pub reserve_usd: f64,
    pub volume_24h: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
pub enum Chain {
    Ethereum,
    Solana,
    Bsc,
    Other,
}

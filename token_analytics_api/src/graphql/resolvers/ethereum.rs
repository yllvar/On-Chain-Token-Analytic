use async_graphql::{Context, Result, FieldResult};
use web3::types::U256;

use crate::models::{Chain, Token, TokenAnalytics};
use crate::rpc::ethereum::EthereumClient;

pub async fn get_token_info(
    _ctx: &Context<'_>,
    address: String,
) -> FieldResult<Token> {
    let rpc_url = "https://mainnet.infura.io/v3/YOUR-PROJECT-ID";
    let client = EthereumClient::new(rpc_url).await?;
    let (name, symbol, decimals) = client.get_token_info(&address).await?;
    let total_supply = client.get_token_supply(&address).await?.as_u128() as f64;

    Ok(Token {
        address,
        name: Some(name),
        symbol: Some(symbol),
        chain: Chain::Ethereum.to_string(),
        decimals: Some(decimals),
        total_supply: Some(total_supply.to_string()),
        price_usd: None,
        price_change_24h: None,
        volume_24h: None,
        market_cap: None,
    })
}

pub async fn get_token_analytics(
    _ctx: &Context<'_>,
    address: String,
) -> FieldResult<TokenAnalytics> {
    let rpc_url = "https://mainnet.infura.io/v3/YOUR-PROJECT-ID";
    let client = EthereumClient::new(rpc_url).await?;
    let (name, symbol, decimals) = client.get_token_info(&address).await?;
    let total_supply = client.get_token_supply(&address).await?.as_u128() as f64;
    
    let token = Token {
        address: address.clone(),
        name: Some(name),
        symbol: Some(symbol),
        chain: Chain::Ethereum.to_string(),
        decimals: Some(decimals),
        total_supply: Some(total_supply.to_string()),
        price_usd: None,
        price_change_24h: None,
        volume_24h: None,
        market_cap: None,
    };
    
    // In a real implementation, fetch holders, transfers, and liquidity data
    // This is a simplified version
    Ok(TokenAnalytics {
        token,
        holders: vec![],
        transfers: vec![],
        liquidity_pools: vec![],
    })
}

// Helper function to convert U256 to f64 with decimals
fn format_amount(amount: U256, decimals: u8) -> f64 {
    let decimals = 10f64.powi(decimals as i32);
    amount.low_u128() as f64 / decimals
}

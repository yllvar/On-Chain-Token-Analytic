use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub redis_url: String,
    pub server_addr: SocketAddr,
    pub eth_rpc_url: String,
    pub sol_rpc_url: String,
    pub cache_ttl: u64, // in seconds
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port: u16 = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "4000".to_string())
            .parse()
            .unwrap_or(4000);

        Ok(Self {
            redis_url,
            server_addr: format!("{}:{}", server_host, server_port).parse().unwrap(),
            eth_rpc_url: env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set"),
            sol_rpc_url: env::var("SOL_RPC_URL").expect("SOL_RPC_URL must be set"),
            cache_ttl: env::var("CACHE_TTL_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
        })
    }
}

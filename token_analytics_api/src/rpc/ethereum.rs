use web3::{
    types::{Address, U256, H160},
    contract::{Contract, Options},
    transports::Http,
    Web3,
};
use anyhow::{Result, anyhow};
use std::str::FromStr;
use std::sync::Arc;

pub struct EthereumClient {
    web3: Web3<Http>,
}

impl EthereumClient {
    pub async fn new(rpc_url: &str) -> Result<Self> {
        let transport = Http::new(rpc_url)
            .map_err(|e| anyhow!("Failed to create HTTP transport: {}", e))?;
        let web3 = Web3::new(transport);
        
        Ok(Self { web3 })
    }
    
    fn get_contract(&self, address: &str) -> Result<Contract<Http>> {
        let address = address.parse::<Address>()
            .map_err(|e| anyhow!("Invalid address: {}", e))?;
            
        // ABI for ERC20
        let abi = include_bytes!("erc20.abi.json");
        
        Ok(Contract::from_json(self.web3.eth(), address, abi)
            .map_err(|e| anyhow!("Failed to create contract: {}", e))?)
    }

    pub async fn get_token_info(&self, address: &str) -> Result<(String, String, u8)> {
        let contract = self.get_contract(address)?;
        
        // Call name()
        let name: String = contract
            .query("name", (), None, Options::default(), None)
            .await
            .map_err(|e| anyhow!("Failed to get token name: {}", e))?;
        
        // Call symbol()
        let symbol: String = contract
            .query("symbol", (), None, Options::default(), None)
            .await
            .map_err(|e| anyhow!("Failed to get token symbol: {}", e))?;
        
        // Call decimals()
        let decimals: u8 = contract
            .query("decimals", (), None, Options::default(), None)
            .await
            .map_err(|e| anyhow!("Failed to get token decimals: {}", e))?;
        
        Ok((name, symbol, decimals))
    }

    pub async fn get_token_balance(&self, token_address: &str, holder_address: &str) -> Result<U256> {
        let contract = self.get_contract(token_address)?;
        let holder = holder_address.parse::<Address>()
            .map_err(|e| anyhow!("Invalid holder address: {}", e))?;
        
        // Call balanceOf(address)
        let balance: U256 = contract
            .query("balanceOf", holder, None, Options::default(), None)
            .await
            .map_err(|e| anyhow!("Failed to get token balance: {}", e))?;
        
        Ok(balance)
    }

    pub async fn get_token_supply(&self, token_address: &str) -> Result<U256> {
        let contract = self.get_contract(token_address)?;
        
        // Call totalSupply()
        let supply: U256 = contract
            .query("totalSupply", (), None, Options::default(), None)
            .await
            .map_err(|e| anyhow!("Failed to get token supply: {}", e))?;
        
        Ok(supply)
    }
}

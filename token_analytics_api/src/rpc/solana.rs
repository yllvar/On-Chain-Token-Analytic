// In src/rpc/solana.rs
use solana_client::{
    rpc_client::RpcClient,
    rpc_request::TokenAccountsFilter,
};
use solana_sdk::{
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_token::state::Account as TokenAccount;
use anyhow::{Result, anyhow};
use std::str::FromStr;

pub struct SolanaClient {
    client: RpcClient,
}

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: RpcClient::new(rpc_url.to_string()),
        }
    }

    pub async fn get_token_balance(&self, mint_address: &str, owner_address: &str) -> Result<u64> {
        let mint_pubkey = Pubkey::from_str(mint_address)?;
        let owner_pubkey = Pubkey::from_str(owner_address)?;
        
        // Get token accounts by owner and mint
        let accounts = self.client.get_token_accounts_by_owner(
            &owner_pubkey,
            TokenAccountsFilter::Mint(mint_pubkey),
        )?;
        
        // Sum up all token account balances for this mint
        let mut total_balance = 0;
        for account in accounts {
            // Get the account data
            let account_data = self.client.get_account_data(&account.pubkey)?;
            if let Ok(token_account) = TokenAccount::unpack(&account_data) {
                total_balance += token_account.amount;
            }
        }
        
        Ok(total_balance)
    }

    pub async fn get_token_supply(&self, mint_address: &str) -> Result<u64> {
        let mint_pubkey = Pubkey::from_str(mint_address)?;
        let token_supply = self.client
            .get_token_supply(&mint_pubkey)
            .map_err(|e| anyhow!("Failed to get token supply: {}", e))?;
            
        // Convert the string amount to u64
        token_supply.amount.parse::<u64>()
            .map_err(|e| anyhow!("Failed to parse token supply: {}", e))
    }

    pub async fn get_token_info(&self, mint_address: &str) -> Result<(String, String, u8)> {
        // In a real implementation, you would fetch the token metadata
        // For now, return placeholder data
        Ok(("Token".to_string(), "TKN".to_string(), 9))
    }
}
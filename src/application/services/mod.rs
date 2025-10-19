//! Application services - Business logic orchestration
//!
//! This module contains the application services that coordinate between
//! the domain layer and infrastructure layer to provide high-level
//! business operations.

use crate::domain::types::Pubkey;
use crate::infrastructure::rpc::SolanaRpcClient;
use std::error::Error;

/// Account service for handling account-related operations
pub struct AccountService {
    rpc_client: SolanaRpcClient,
}

impl AccountService {
    /// Create a new account service
    pub fn new(rpc_client: SolanaRpcClient) -> Self {
        Self { rpc_client }
    }

    /// Get account balance with error handling
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, Box<dyn Error>> {
        self.rpc_client
            .get_balance(pubkey)
            .await
            .map_err(Into::into)
    }

    /// Get account information with error handling
    pub async fn get_account_info(
        &self,
        pubkey: &Pubkey,
    ) -> Result<Option<crate::infrastructure::rpc::Account>, Box<dyn Error>> {
        self.rpc_client
            .get_account_info(pubkey)
            .await
            .map_err(Into::into)
    }

    /// Get multiple account balances efficiently
    pub async fn get_multiple_balances(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<u64>>, Box<dyn Error>> {
        let accounts = self.rpc_client.get_multiple_accounts(pubkeys).await?;
        Ok(accounts
            .into_iter()
            .map(|acc| acc.map(|a| a.lamports))
            .collect())
    }
}

/// Transaction service for handling transaction operations
pub struct TransactionService {
    rpc_client: SolanaRpcClient,
}

impl TransactionService {
    /// Create a new transaction service
    pub fn new(rpc_client: SolanaRpcClient) -> Self {
        Self { rpc_client }
    }

    /// Get the latest blockhash for transaction building
    pub async fn get_latest_blockhash(
        &self,
    ) -> Result<crate::infrastructure::rpc::LatestBlockhash, Box<dyn Error>> {
        self.rpc_client
            .get_latest_blockhash()
            .await
            .map_err(Into::into)
    }

    /// Send a transaction to the network
    pub async fn send_transaction(
        &self,
        transaction: &str,
    ) -> Result<crate::domain::types::Signature, Box<dyn Error>> {
        self.rpc_client
            .send_transaction(transaction)
            .await
            .map_err(Into::into)
    }

    /// Get current block height
    pub async fn get_block_height(&self) -> Result<u64, Box<dyn Error>> {
        self.rpc_client.get_block_height().await.map_err(Into::into)
    }
}

/// Network service for network-related operations
pub struct NetworkService {
    rpc_client: SolanaRpcClient,
}

impl NetworkService {
    /// Create a new network service
    pub fn new(rpc_client: SolanaRpcClient) -> Self {
        Self { rpc_client }
    }

    /// Check if the network is reachable
    pub async fn health_check(&self) -> Result<bool, Box<dyn Error>> {
        match self.rpc_client.get_block_height().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get network status information
    pub async fn get_network_status(&self) -> Result<NetworkStatus, Box<dyn Error>> {
        let block_height = self.rpc_client.get_block_height().await?;
        let latest_blockhash = self.rpc_client.get_latest_blockhash().await?;

        Ok(NetworkStatus {
            is_connected: true,
            block_height,
            latest_blockhash,
        })
    }
}

/// Network status information
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub is_connected: bool,
    pub block_height: u64,
    pub latest_blockhash: crate::infrastructure::rpc::LatestBlockhash,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::rpc::Network;

    #[test]
    fn test_service_creation() {
        let rpc_client =
            crate::infrastructure::rpc::RpcClientBuilder::new(Network::Devnet.endpoint()).build();

        let _account_service = AccountService::new(rpc_client.clone());
        let _transaction_service = TransactionService::new(rpc_client.clone());
        let _network_service = NetworkService::new(rpc_client);

        // Test that services are created successfully
        assert!(true); // Placeholder assertion
    }
}

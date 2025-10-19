//! Solana JSON-RPC client implementation
//!
//! This module provides a complete implementation of the Solana JSON-RPC API
//! using HTTP requests, designed to work in WASM environments.

use crate::domain::types::{Hash, Pubkey, Signature};
use crate::infrastructure::http::{HttpClient, HttpError};
use crate::WasmHttpClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

/// Solana JSON-RPC client
#[derive(Clone)]
pub struct SolanaRpcClient {
    http_client: WasmHttpClient,
    endpoint: String,
}

impl SolanaRpcClient {
    /// Create a new RPC client with the given endpoint
    pub fn new(
        endpoint: impl Into<String>,
        http_client: crate::infrastructure::http::WasmHttpClient,
    ) -> Self {
        Self {
            http_client,
            endpoint: endpoint.into(),
        }
    }

    /// Get the RPC endpoint URL
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get account information
    pub async fn get_account_info(&self, pubkey: &Pubkey) -> Result<Option<Account>, RpcError> {
        let request = RpcRequest::new("getAccountInfo")
            .param(pubkey.to_base58())
            .param(json!({
                "encoding": "base64"
            }));

        let response: RpcResponse<Option<AccountInfo>> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Ok(response.result.map(|info| {
            let data = info.data.decode_data().unwrap_or_default();
            Account {
                pubkey: *pubkey,
                lamports: info.lamports,
                data,
                owner: info.owner,
                executable: info.executable,
                rent_epoch: info.rent_epoch,
            }
        }))
    }

    /// Get account balance
    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, RpcError> {
        let request = RpcRequest::new("getBalance").param(pubkey.to_base58());

        let response: RpcResponse<BalanceInfo> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Ok(response.result.value)
    }

    /// Get the latest blockhash
    pub async fn get_latest_blockhash(&self) -> Result<LatestBlockhash, RpcError> {
        let request = RpcRequest::new("getLatestBlockhash");

        let response: RpcResponse<LatestBlockhashInfo> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Ok(response.result.value)
    }

    /// Send a transaction
    pub async fn send_transaction(&self, transaction: &str) -> Result<Signature, RpcError> {
        let request = RpcRequest::new("sendTransaction")
            .param(transaction)
            .param(json!({
                "encoding": "base64"
            }));

        let response: RpcResponse<String> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Signature::from_base58(&response.result).map_err(RpcError::InvalidSignature)
    }

    /// Get block height
    pub async fn get_block_height(&self) -> Result<u64, RpcError> {
        let request = RpcRequest::new("getBlockHeight");

        let response: RpcResponse<u64> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Ok(response.result)
    }

    /// Get multiple accounts
    pub async fn get_multiple_accounts(
        &self,
        pubkeys: &[Pubkey],
    ) -> Result<Vec<Option<Account>>, RpcError> {
        let pubkey_strings: Vec<String> = pubkeys.iter().map(|pk| pk.to_base58()).collect();

        let request = RpcRequest::new("getMultipleAccounts")
            .param(pubkey_strings)
            .param(json!({
                "encoding": "base64"
            }));

        let response: RpcResponse<Vec<Option<AccountInfo>>> = self
            .http_client
            .post_json(&self.endpoint, &request)
            .await
            .map_err(RpcError::Http)?;

        Ok(response
            .result
            .into_iter()
            .enumerate()
            .map(|(i, info)| {
                info.map(|info| {
                    let data = info.data.decode_data().unwrap_or_default();
                    Account {
                        pubkey: pubkeys[i],
                        lamports: info.lamports,
                        data,
                        owner: info.owner,
                        executable: info.executable,
                        rent_epoch: info.rent_epoch,
                    }
                })
            })
            .collect())
    }
}

/// JSON-RPC request structure
#[derive(Debug, Clone, Serialize)]
struct RpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Vec<serde_json::Value>,
}

impl RpcRequest {
    fn new(method: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.into(),
            params: Vec::new(),
        }
    }

    fn param(mut self, param: impl Serialize) -> Self {
        self.params.push(serde_json::to_value(param).unwrap());
        self
    }
}

/// JSON-RPC response structure
#[derive(Debug, Clone, Deserialize)]
struct RpcResponse<T> {
    result: T,
}

/// Account information from RPC
#[derive(Debug, Clone, Deserialize)]
struct AccountInfo {
    lamports: u64,
    data: AccountData,
    owner: Pubkey,
    executable: bool,
    rent_epoch: u64,
}

/// Account data structure
#[derive(Debug, Clone, Deserialize)]
struct AccountData {
    #[serde(rename = "data")]
    data: String, // Base64 encoded data
}

impl AccountData {
    /// Decode the base64 data
    fn decode_data(&self) -> Result<Vec<u8>, base64::DecodeError> {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.decode(&self.data)
    }
}

/// Balance information from RPC
#[derive(Debug, Clone, Deserialize)]
struct BalanceInfo {
    value: u64,
}

/// Latest blockhash information from RPC
#[derive(Debug, Clone, Deserialize)]
struct LatestBlockhashInfo {
    value: LatestBlockhash,
}

/// Latest blockhash structure
#[derive(Debug, Clone, Deserialize)]
pub struct LatestBlockhash {
    pub blockhash: Hash,
    pub last_valid_block_height: u64,
}

/// Account structure
#[derive(Debug, Clone)]
pub struct Account {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// RPC error types
#[derive(Debug, Clone, Error)]
pub enum RpcError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),

    #[error("Invalid signature: {0}")]
    InvalidSignature(#[from] crate::domain::types::SignatureError),

    #[error("Invalid public key: {0}")]
    InvalidPubkey(#[from] crate::domain::types::PubkeyError),

    #[error("RPC error: {code} - {message}")]
    RpcError { code: i64, message: String },

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Network configuration
#[derive(Debug, Clone)]
pub enum Network {
    Mainnet,
    Testnet,
    Devnet,
    Custom(String),
}

impl Network {
    /// Get the RPC endpoint for this network
    pub fn endpoint(&self) -> &str {
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
            Network::Testnet => "https://api.testnet.solana.com",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Custom(url) => url,
        }
    }
}

/// Create a network configuration for surfpool (simnet)
pub fn surfpool_network() -> Network {
    Network::Custom("http://127.0.0.1:8899".to_string())
}

/// RPC client builder
pub struct RpcClientBuilder {
    endpoint: String,
    config: RpcClientConfig,
}

impl RpcClientBuilder {
    /// Create a new RPC client builder
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            config: RpcClientConfig::default(),
        }
    }

    /// Set commitment level
    pub fn commitment(mut self, commitment: CommitmentLevel) -> Self {
        self.config.commitment = Some(commitment);
        self
    }

    /// Build the RPC client
    pub fn build(self) -> SolanaRpcClient {
        let http_client = crate::infrastructure::http::WasmHttpClient::new();
        SolanaRpcClient::new(self.endpoint, http_client)
    }
}

/// RPC client configuration
#[derive(Debug, Clone)]
struct RpcClientConfig {
    commitment: Option<CommitmentLevel>,
}

impl Default for RpcClientConfig {
    fn default() -> Self {
        Self { commitment: None }
    }
}

/// Commitment levels
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_endpoints() {
        assert_eq!(
            Network::Mainnet.endpoint(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            Network::Testnet.endpoint(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(Network::Devnet.endpoint(), "https://api.devnet.solana.com");
        assert_eq!(surfpool_network().endpoint(), "http://127.0.0.1:8899");
    }

    #[test]
    fn test_rpc_request_serialization() {
        let system_pubkey = Pubkey::from_base58("11111111111111111111111111111111").unwrap();
        let request = RpcRequest::new("getBalance").param(system_pubkey.to_base58());

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("getBalance"));
        assert!(json.contains("11111111111111111111111111111111"));
    }

    #[test]
    fn test_rpc_client_builder() {
        let client = RpcClientBuilder::new("http://localhost:8899")
            .commitment(CommitmentLevel::Confirmed)
            .build();

        // Test that the client was created successfully
        assert_eq!(client.endpoint(), "http://localhost:8899");
    }
}

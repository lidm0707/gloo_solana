//! gloo_solana - WASM-compatible Solana SDK rewrite using gloo_net
//!
//! This library provides a WASM-compatible implementation of Solana functionality
//! using HTTP requests instead of direct TCP connections. It's designed to work
//! seamlessly with Dioxus and other web frameworks.
//!
//! # Features
//!
//! - HTTP-based JSON-RPC client for Solana
//! - Support for surfpool (simnet) and mainnet
//! - WASM-compatible implementation
//! - DDD architecture with SOLID principles
//! - Dioxus integration support
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use gloo_solana::{RpcClientBuilder, surfpool_network, CommitmentLevel, constants::SYSTEM_PROGRAM_ID};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RpcClientBuilder::new(surfpool_network().endpoint())
//!         .commitment(CommitmentLevel::Confirmed)
//!         .build();
//!
//!     let balance = client.get_balance(&SYSTEM_PROGRAM_ID).await?;
//!     println!("System program balance: {} lamports", balance);
//!
//!     Ok(())
//! }
//! ```

pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::types::constants;
pub use domain::types::{Hash, HashError, Pubkey, PubkeyError, Signature, SignatureError};
pub use infrastructure::http::HttpError;
#[cfg(target_arch = "wasm32")]
pub use infrastructure::http::WasmHttpClient;
pub use infrastructure::rpc::{
    surfpool_network, Account, CommitmentLevel, LatestBlockhash, Network, RpcClientBuilder,
    RpcError, SolanaRpcClient,
};

#[cfg(feature = "dioxus")]
pub mod dioxus_integration;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default commitment level for RPC calls
pub const DEFAULT_COMMITMENT: CommitmentLevel = CommitmentLevel::Confirmed;

/// Create a new RPC client for the specified network
pub fn create_client(network: Network) -> SolanaRpcClient {
    RpcClientBuilder::new(network.endpoint())
        .commitment(DEFAULT_COMMITMENT)
        .build()
}

/// Create a new RPC client for surfpool (simnet)
pub fn create_surfpool_client() -> SolanaRpcClient {
    create_client(surfpool_network())
}

/// Create a new RPC client for mainnet
pub fn create_mainnet_client() -> SolanaRpcClient {
    create_client(Network::Mainnet)
}

/// Create a new RPC client for devnet
pub fn create_devnet_client() -> SolanaRpcClient {
    create_client(Network::Devnet)
}

/// Create a new RPC client for testnet
pub fn create_testnet_client() -> SolanaRpcClient {
    create_client(Network::Testnet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_create_clients() {
        let _surf_client = create_surfpool_client();
        let _mainnet_client = create_mainnet_client();
        let _devnet_client = create_devnet_client();
        let _testnet_client = create_testnet_client();
    }

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
}

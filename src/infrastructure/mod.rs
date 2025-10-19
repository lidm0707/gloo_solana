//! Infrastructure layer - External integrations and technical implementations
//!
//! This module contains the infrastructure components that handle external
//! integrations, such as HTTP clients, RPC clients, and other technical
//! concerns required to interact with the Solana network.

pub mod http;
pub mod rpc;

// Re-export commonly used infrastructure components
pub use http::{HttpClient, HttpClientBuilder, HttpError, WasmHttpClient};
pub use rpc::{
    surfpool_network, Account, CommitmentLevel, LatestBlockhash, Network, RpcClientBuilder,
    RpcError, SolanaRpcClient,
};

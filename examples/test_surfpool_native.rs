//! Example: Testing surfpool connection (native version)
//!
//! This example demonstrates how to use the gloo_solana library to connect
//! to a surfpool (simnet) instance and perform basic operations without WASM dependencies.

use gloo_solana::{
    constants::SYSTEM_PROGRAM_ID, surfpool_network, CommitmentLevel, RpcClientBuilder,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌊 Starting surfpool connection test...");

    // Create RPC client for surfpool
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    println!(
        "✅ Created RPC client for surfpool at: {}",
        surfpool_network().endpoint()
    );

    // Create a tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new()?;

    // Test 1: Get version information
    rt.block_on(test_get_version(&client))?;

    // Test 2: Get latest blockhash
    rt.block_on(test_get_latest_blockhash(&client))?;

    // Test 3: Get system program balance
    rt.block_on(test_get_balance(&client))?;

    // Test 4: Get system program account info
    rt.block_on(test_get_account_info(&client))?;

    // Test 5: Get block height
    rt.block_on(test_get_block_height(&client))?;

    println!("🎉 All tests completed successfully!");

    Ok(())
}

async fn test_get_version(client: &gloo_solana::SolanaRpcClient) -> Result<(), Box<dyn Error>> {
    println!("📋 Testing getVersion...");

    // This would require adding getVersion method to our RPC client
    // For now, we'll test with getBlockHeight as a basic connectivity test
    let block_height = client.get_block_height().await?;
    println!(
        "✅ Connected to surfpool. Current block height: {}",
        block_height
    );

    Ok(())
}

async fn test_get_latest_blockhash(
    client: &gloo_solana::SolanaRpcClient,
) -> Result<(), Box<dyn Error>> {
    println!("🔗 Testing getLatestBlockhash...");

    let latest_blockhash = client.get_latest_blockhash().await?;
    println!(
        "✅ Latest blockhash: {} (valid until block {})",
        latest_blockhash.blockhash, latest_blockhash.last_valid_block_height
    );

    Ok(())
}

async fn test_get_balance(client: &gloo_solana::SolanaRpcClient) -> Result<(), Box<dyn Error>> {
    println!("💰 Testing getBalance for system program...");

    let balance = client.get_balance(&SYSTEM_PROGRAM_ID).await?;
    println!("✅ System program balance: {} lamports", balance);

    Ok(())
}

async fn test_get_account_info(
    client: &gloo_solana::SolanaRpcClient,
) -> Result<(), Box<dyn Error>> {
    println!("📊 Testing getAccountInfo for system program...");

    let account = client.get_account_info(&SYSTEM_PROGRAM_ID).await?;

    match account {
        Some(account) => {
            println!("✅ System program account found:");
            println!("   - Lamports: {}", account.lamports);
            println!("   - Owner: {}", account.owner);
            println!("   - Executable: {}", account.executable);
            println!("   - Data length: {} bytes", account.data.len());
            println!("   - Rent epoch: {}", account.rent_epoch);
        }
        None => {
            println!("ℹ️  System program account not found (this is unexpected)");
        }
    }

    Ok(())
}

async fn test_get_block_height(
    client: &gloo_solana::SolanaRpcClient,
) -> Result<(), Box<dyn Error>> {
    println!("📏 Testing getBlockHeight...");

    let block_height = client.get_block_height().await?;
    println!("✅ Current block height: {}", block_height);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_solana::Network;

    #[test]
    fn test_surfpool_endpoint() {
        let endpoint = surfpool_network().endpoint();
        assert_eq!(endpoint, "http://127.0.0.1:8899");
    }

    #[test]
    fn test_client_creation() {
        let client = RpcClientBuilder::new(surfpool_network().endpoint())
            .commitment(CommitmentLevel::Confirmed)
            .build();

        // Just test that the client is created successfully
        // Actual network calls would be tested in integration tests
        assert_eq!(client.endpoint(), "http://127.0.0.1:8899");
    }

    #[tokio::test]
    #[ignore] // Ignore by default since it requires surfpool to be running
    async fn test_surfpool_connectivity() {
        let client = RpcClientBuilder::new(surfpool_network().endpoint())
            .commitment(CommitmentLevel::Confirmed)
            .build();

        // This test will fail if surfpool is not running
        let result = client.get_block_height().await;
        assert!(result.is_ok());
    }
}

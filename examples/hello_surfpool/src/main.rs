//! Hello Surfpool Example
//!
//! This example demonstrates basic account operations using gloo_solana
//! It will create an account with a "hello surfpool" message and read it back
//! Since we're only using gloo_solana (HTTP-based), we'll work with existing
//! accounts on surfpool rather than deploying programs.

use gloo_solana::{
    constants::{SYSTEM_PROGRAM_ID, SYSVAR_CLOCK_ID},
    surfpool_network, CommitmentLevel, Pubkey, RpcClientBuilder,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Account data structure for our hello message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloAccount {
    /// Account name
    pub name: String,
    /// Message
    pub message: String,
    /// Unix timestamp when created
    pub created_at: u64,
}

impl HelloAccount {
    /// Create a new hello account
    pub fn new(name: String, message: String) -> Self {
        Self {
            name,
            message,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Serialize to bytes for storage
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌊 Hello Surfpool Example");
    println!("==========================");

    // Create RPC client for surfpool
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    println!("✅ Connected to surfpool at: {}", client.endpoint());

    // Test basic connectivity
    test_connectivity(&client).await?;

    // Create our hello account data
    let hello_account = HelloAccount::new(
        "hello surf".to_string(),
        "hello surfpool from gloo_solana!".to_string(),
    );

    println!("\n📝 Created Hello Account:");
    println!("   Name: {}", hello_account.name);
    println!("   Message: {}", hello_account.message);
    println!("   Created: {}", hello_account.created_at);

    // Serialize account data
    let account_bytes = hello_account.to_bytes()?;
    println!("   Serialized size: {} bytes", account_bytes.len());

    // Since we can't deploy programs with gloo_solana only, we'll demonstrate
    // working with existing accounts on surfpool

    println!("\n🔍 Exploring Existing Accounts:");

    // Check system program account
    explore_system_program(&client).await?;

    // Check clock sysvar
    explore_clock_sysvar(&client).await?;

    // Demonstrate account data simulation
    demonstrate_account_simulation(&hello_account)?;

    println!("\n✅ Hello Surfpool example completed successfully!");
    println!("\n💡 Note: This example demonstrates account reading and data handling.");
    println!("   To deploy actual programs, you would need the full solana-sdk.");

    Ok(())
}

/// Test basic connectivity to surfpool
async fn test_connectivity(client: &gloo_solana::SolanaRpcClient) -> Result<(), Box<dyn Error>> {
    println!("\n🔌 Testing Connectivity...");

    // Get latest blockhash
    let latest_blockhash = client.get_latest_blockhash().await?;
    println!("✅ Latest Blockhash: {}", latest_blockhash.blockhash);
    println!(
        "   Valid Until Block: {}",
        latest_blockhash.last_valid_block_height
    );

    // Get block height
    let block_height = client.get_block_height().await?;
    println!("✅ Current Block Height: {}", block_height);

    Ok(())
}

/// Explore system program account
async fn explore_system_program(
    client: &gloo_solana::SolanaRpcClient,
) -> Result<(), Box<dyn Error>> {
    println!("\n🏛️  System Program Account:");

    // Get system program info
    let account = client.get_account_info(&SYSTEM_PROGRAM_ID).await?;

    match account {
        Some(acc) => {
            println!("   Pubkey: {}", acc.pubkey);
            println!("   Lamports: {}", acc.lamports);
            println!("   Owner: {}", acc.owner);
            println!("   Executable: {}", acc.executable);
            println!("   Data Length: {} bytes", acc.data.len());

            // Try to interpret some data
            if acc.data.len() >= 8 {
                let first_8_bytes = &acc.data[..8];
                println!("   First 8 bytes: {:?}", first_8_bytes);

                // Interpret as little-endian u64 if possible
                if first_8_bytes.len() == 8 {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(first_8_bytes);
                    let value = u64::from_le_bytes(bytes);
                    println!("   As u64 (LE): {}", value);
                }
            }
        }
        None => {
            println!("   ⚠️  System program account not found");
        }
    }

    Ok(())
}

/// Explore clock sysvar
async fn explore_clock_sysvar(client: &gloo_solana::SolanaRpcClient) -> Result<(), Box<dyn Error>> {
    println!("\n⏰ Clock Sysvar:");

    // Get clock sysvar info
    let account = client.get_account_info(&SYSVAR_CLOCK_ID).await?;

    match account {
        Some(acc) => {
            println!("   Pubkey: {}", acc.pubkey);
            println!("   Lamports: {}", acc.lamports);
            println!("   Owner: {}", acc.owner);
            println!("   Data Length: {} bytes", acc.data.len());

            // Clock sysvar contains Unix timestamp, slot, and epoch info
            if acc.data.len() >= 40 {
                // Clock sysvar structure:
                // - 8 bytes: Unix timestamp (u64)
                // - 8 bytes: Bank start slot (u64)
                // - 8 bytes: Epoch start slot (u64)
                // - 8 bytes: Leader schedule epoch (u64)
                // - 8 bytes: Unix timestamp of epoch start (u64)

                let mut timestamp_bytes = [0u8; 8];
                timestamp_bytes.copy_from_slice(&acc.data[0..8]);
                let timestamp = u64::from_le_bytes(timestamp_bytes);

                let mut slot_bytes = [0u8; 8];
                slot_bytes.copy_from_slice(&acc.data[8..16]);
                let slot = u64::from_le_bytes(slot_bytes);

                println!("   Current Unix Timestamp: {}", timestamp);
                println!("   Current Slot: {}", slot);

                // Convert timestamp to human-readable format
                let datetime = std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
                if let Ok(datetime) = datetime.elapsed() {
                    println!("   Time: {} seconds ago", datetime.as_secs());
                }
            }
        }
        None => {
            println!("   ⚠️  Clock sysvar not found");
        }
    }

    Ok(())
}

/// Demonstrate account data simulation
fn demonstrate_account_simulation(hello_account: &HelloAccount) -> Result<(), Box<dyn Error>> {
    println!("\n🎭 Account Data Simulation:");

    // Serialize our account
    let serialized = hello_account.to_bytes()?;
    println!("   Serialized data: {} bytes", serialized.len());

    // Show hex representation
    let hex_str: String = serialized.iter().map(|b| format!("{:02x}", b)).collect();
    println!("   Hex: {}", hex_str);

    // Show base64 representation (how Solana stores data)
    let base64_str = base64::encode(&serialized);
    println!("   Base64: {}", base64_str);

    // Deserialize back
    let deserialized = HelloAccount::from_bytes(&serialized)?;
    println!("   ✅ Roundtrip successful: {}", deserialized.message);

    // Simulate multiple accounts with different names
    let sample_accounts = vec![
        HelloAccount::new("user1".to_string(), "Hello from user1!".to_string()),
        HelloAccount::new("user2".to_string(), "Greetings from user2!".to_string()),
        HelloAccount::new("admin".to_string(), "System message".to_string()),
    ];

    println!("\n👥 Sample Account Data:");
    for (i, account) in sample_accounts.iter().enumerate() {
        let serialized = account.to_bytes()?;
        println!(
            "   Account {}: {} -> {} bytes",
            i + 1,
            account.name,
            serialized.len()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_account_serialization() {
        let account = HelloAccount::new("test".to_string(), "test message".to_string());

        let serialized = account.to_bytes().unwrap();
        let deserialized = HelloAccount::from_bytes(&serialized).unwrap();

        assert_eq!(account.name, deserialized.name);
        assert_eq!(account.message, deserialized.message);
        assert_eq!(account.created_at, deserialized.created_at);
    }

    #[test]
    fn test_hello_account_creation() {
        let account = HelloAccount::new("hello surf".to_string(), "hello surfpool".to_string());

        assert_eq!(account.name, "hello surf");
        assert_eq!(account.message, "hello surfpool");
        assert!(account.created_at > 0);
    }

    #[tokio::test]
    async fn test_surfpool_connectivity() {
        let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();

        // This test requires surfpool to be running
        let result = client.get_block_height().await;

        // We'll just verify the call doesn't panic
        // It may fail if surfpool isn't running, which is ok for tests
        match result {
            Ok(height) => assert!(height > 0),
            Err(_) => println!("Surfpool not running - skipping connectivity test"),
        }
    }
}

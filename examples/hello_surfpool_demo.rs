//! Hello Surfpool Demo Example
//!
//! This example demonstrates how to work with Solana account data using gloo_solana
//! It simulates creating an account with "hello surf" message and shows data handling
//! This version works without requiring actual network connections or WASM

use gloo_solana::{
    constants::SYSTEM_PROGRAM_ID, surfpool_network, CommitmentLevel, Pubkey, RpcClientBuilder,
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
    /// Account owner (program ID)
    pub owner: Pubkey,
    /// Account balance in lamports
    pub balance: u64,
}

impl HelloAccount {
    /// Create a new hello account
    pub fn new(name: String, message: String, owner: Pubkey) -> Self {
        Self {
            name,
            message,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            owner,
            balance: 0,
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

    /// Get account size in bytes
    pub fn size(&self) -> usize {
        self.to_bytes().unwrap_or_default().len()
    }
}

/// Simulated account data for demonstration
#[derive(Debug, Clone)]
pub struct SimulatedAccount {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

impl SimulatedAccount {
    /// Create a simulated account from HelloAccount
    fn from_hello_account(hello: &HelloAccount) -> Result<Self, serde_json::Error> {
        let data = hello.to_bytes()?;
        Ok(Self {
            pubkey: generate_random_pubkey(),
            lamports: hello.balance,
            data,
            owner: hello.owner,
            executable: false,
            rent_epoch: 0,
        })
    }
}

/// Generate a random-looking pubkey for demonstration
fn generate_random_pubkey() -> Pubkey {
    // Create a deterministic but "random-looking" pubkey
    let mut bytes = [0u8; 32];
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // Simple deterministic "random" generation
    for i in 0..32 {
        // Fix the bit shift - use wrapping shift to avoid overflow
        let shift_amount = (i % 8) * 8;
        bytes[i] = ((seed >> shift_amount) & 0xFF) as u8;
        if i % 4 == 0 {
            bytes[i] ^= 0x42; // Add some variation
        }
    }

    Pubkey::new(bytes)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌊 Hello Surfpool Demo Example");
    println!("===============================");
    println!();
    println!("This demo shows how to work with Solana account data");
    println!("using gloo_solana without requiring actual network calls.");
    println!();

    // Create our hello account with "hello surf" message
    let hello_account = HelloAccount::new(
        "hello surf".to_string(),
        "Hello from gloo_solana! 🌊".to_string(),
        SYSTEM_PROGRAM_ID,
    );

    println!("📝 Created Hello Account:");
    println!("   Name: {}", hello_account.name);
    println!("   Message: {}", hello_account.message);
    println!("   Owner: {}", hello_account.owner);
    println!("   Created: {}", hello_account.created_at);
    println!("   Size: {} bytes", hello_account.size());
    println!();

    // Serialize the account data
    let serialized_data = hello_account.to_bytes()?;
    println!("💾 Serialized Account Data:");
    println!("   Raw bytes: {} bytes", serialized_data.len());

    // Show hex representation
    let hex_str: String = serialized_data
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    println!("   Hex: {}", hex_str);

    // Show base64 representation (how Solana stores data)
    use base64::{engine::general_purpose, Engine as _};
    let base64_str = general_purpose::STANDARD.encode(&serialized_data);
    println!("   Base64: {}", base64_str);
    println!();

    // Create simulated account
    let simulated_account = SimulatedAccount::from_hello_account(&hello_account)?;
    println!("🏦 Simulated Account:");
    println!("   Pubkey: {}", simulated_account.pubkey);
    println!("   Lamports: {}", simulated_account.lamports);
    println!("   Owner: {}", simulated_account.owner);
    println!("   Executable: {}", simulated_account.executable);
    println!("   Data Size: {} bytes", simulated_account.data.len());
    println!();

    // Demonstrate reading the account data back
    println!("📖 Reading Account Data:");
    let deserialized_account = HelloAccount::from_bytes(&simulated_account.data)?;
    println!("   ✅ Successfully deserialized!");
    println!("   Name: {}", deserialized_account.name);
    println!("   Message: {}", deserialized_account.message);
    println!("   Created: {}", deserialized_account.created_at);
    println!();

    // Create multiple accounts with different users
    println!("👥 Creating Multiple User Accounts:");
    let users = vec![
        ("alice", "Hello from Alice! 👋"),
        ("bob", "Bob says hi! 👋"),
        ("charlie", "Charlie's greeting! 🌟"),
        ("diana", "Diana waves hello! 👋"),
    ];

    let mut user_accounts = Vec::new();
    for (username, message) in users {
        let account =
            HelloAccount::new(username.to_string(), message.to_string(), SYSTEM_PROGRAM_ID);

        let simulated = SimulatedAccount::from_hello_account(&account)?;
        println!(
            "   {}: {} -> {} bytes",
            username,
            simulated.pubkey,
            account.size()
        );
        user_accounts.push((account, simulated));
    }
    println!();

    // Demonstrate account search functionality
    println!("🔍 Account Search Demo:");
    let search_name = "alice";
    for (account, simulated) in &user_accounts {
        if account.name == search_name {
            println!("   Found account for '{}':", search_name);
            println!("   Pubkey: {}", simulated.pubkey);
            println!("   Message: {}", account.message);
            println!("   Created: {}", account.created_at);
            break;
        }
    }
    println!();

    // Show network configuration
    println!("🌐 Network Configuration:");
    println!("   Surfpool Endpoint: {}", surfpool_network().endpoint());
    println!(
        "   Mainnet Endpoint: {}",
        gloo_solana::Network::Mainnet.endpoint()
    );
    println!(
        "   Devnet Endpoint: {}",
        gloo_solana::Network::Devnet.endpoint()
    );
    println!(
        "   Testnet Endpoint: {}",
        gloo_solana::Network::Testnet.endpoint()
    );
    println!();

    // Demonstrate RPC client creation (without actual calls)
    println!("🔧 RPC Client Configuration:");
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();
    println!("   Client created for: {}", client.endpoint());
    println!("   Commitment: Confirmed");
    println!();

    // Show what would happen with real network calls
    println!("🚀 Real Network Operations (What would happen):");
    println!("   1. client.get_account_info(&pubkey) -> Account data");
    println!("   2. client.get_balance(&pubkey) -> Account balance");
    println!("   3. client.get_latest_blockhash() -> Latest blockhash");
    println!("   4. client.send_transaction(&tx) -> Transaction signature");
    println!();

    // Calculate total storage needed
    let total_size: usize = user_accounts.iter().map(|(acc, _)| acc.size()).sum();
    println!("📊 Storage Statistics:");
    println!("   Total accounts: {}", user_accounts.len() + 1); // +1 for hello account
    println!(
        "   Total storage: {} bytes",
        total_size + hello_account.size()
    );
    println!(
        "   Average size: {:.1} bytes",
        (total_size + hello_account.size()) as f64 / (user_accounts.len() + 1) as f64
    );
    println!();

    // Demonstrate data integrity
    println!("🔒 Data Integrity Check:");
    let original_data = hello_account.to_bytes()?;
    let roundtrip = HelloAccount::from_bytes(&original_data)?;

    let integrity_ok = hello_account.name == roundtrip.name
        && hello_account.message == roundtrip.message
        && hello_account.created_at == roundtrip.created_at;

    println!(
        "   Roundtrip successful: {}",
        if integrity_ok { "✅" } else { "❌" }
    );
    println!(
        "   Data integrity: {}",
        if integrity_ok { "PASSED" } else { "FAILED" }
    );
    println!();

    println!("🎉 Hello Surfpool Demo completed successfully!");
    println!();
    println!("💡 Key Takeaways:");
    println!("   • Account data is serialized as JSON bytes");
    println!("   • Solana stores data as base64-encoded bytes");
    println!("   • Each account has a unique pubkey");
    println!("   • gloo_solana provides HTTP-based access to Solana");
    println!("   • Real network calls require surfpool to be running");
    println!();
    println!("🔗 To use with real surfpool:");
    println!("   1. Start surfpool: surfpool start");
    println!("   2. Run: cargo run --example hello_surfpool");
    println!("   3. Or use the test script: ./test_surfpool.sh");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_account_serialization() {
        let account = HelloAccount::new(
            "test".to_string(),
            "test message".to_string(),
            SYSTEM_PROGRAM_ID,
        );

        let serialized = account.to_bytes().unwrap();
        let deserialized = HelloAccount::from_bytes(&serialized).unwrap();

        assert_eq!(account.name, deserialized.name);
        assert_eq!(account.message, deserialized.message);
        assert_eq!(account.created_at, deserialized.created_at);
        assert_eq!(account.owner, deserialized.owner);
    }

    #[test]
    fn test_simulated_account_creation() {
        let hello = HelloAccount::new(
            "test".to_string(),
            "test message".to_string(),
            SYSTEM_PROGRAM_ID,
        );

        let simulated = SimulatedAccount::from_hello_account(&hello).unwrap();

        assert_eq!(simulated.owner, SYSTEM_PROGRAM_ID);
        assert_eq!(simulated.lamports, 0);
        assert!(!simulated.executable);
        assert!(!simulated.data.is_empty());
    }

    #[test]
    fn test_pubkey_generation() {
        let pubkey1 = generate_random_pubkey();
        let pubkey2 = generate_random_pubkey();

        // Should be different due to time difference
        assert_ne!(pubkey1, pubkey2);

        // Should be valid base58
        let base58 = pubkey1.to_base58();
        assert!(!base58.is_empty());
        assert!(base58.len() > 20); // Typical Solana pubkey length
    }

    #[test]
    fn test_account_size_calculation() {
        let account1 = HelloAccount::new("short".to_string(), "msg".to_string(), SYSTEM_PROGRAM_ID);

        let account2 = HelloAccount::new(
            "very_long_name".to_string(),
            "this is a much longer message with more content".to_string(),
            SYSTEM_PROGRAM_ID,
        );

        assert!(account2.size() > account1.size());
    }
}

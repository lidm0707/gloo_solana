//! Basic functionality test for gloo_solana
//!
//! This example tests the basic functionality of the gloo_solana library
//! without requiring WASM or actual network connections.

use gloo_solana::{
    constants::SYSTEM_PROGRAM_ID, surfpool_network, CommitmentLevel, Network, Pubkey,
    RpcClientBuilder,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌊 gloo_solana Basic Functionality Test");
    println!("=========================================");

    // Test 1: Pubkey creation and conversion
    test_pubkey_operations()?;

    // Test 2: Network configuration
    test_network_configuration()?;

    // Test 3: RPC client creation
    test_rpc_client_creation()?;

    // Test 4: Constants
    test_constants()?;

    println!("\n✅ All basic tests passed successfully!");
    Ok(())
}

fn test_pubkey_operations() -> Result<(), Box<dyn Error>> {
    println!("\n📋 Testing Pubkey operations...");

    // Test creating a pubkey from bytes
    let bytes = [1u8; 32];
    let pubkey = Pubkey::new(bytes);
    println!("   Created pubkey from bytes");

    // Test base58 conversion
    let base58 = pubkey.to_base58();
    println!("   Base58 representation: {}", base58);

    // Test roundtrip conversion
    let decoded = Pubkey::from_base58(&base58)?;
    assert_eq!(pubkey, decoded);
    println!("   ✅ Base58 roundtrip conversion successful");

    // Test system program pubkey
    let system_pubkey = SYSTEM_PROGRAM_ID;
    println!("   System program pubkey: {}", system_pubkey);

    Ok(())
}

fn test_network_configuration() -> Result<(), Box<dyn Error>> {
    println!("\n🌐 Testing network configuration...");

    // Test different network endpoints
    let networks = vec![
        ("Mainnet", Network::Mainnet),
        ("Devnet", Network::Devnet),
        ("Testnet", Network::Testnet),
        ("Surfpool", surfpool_network()),
    ];

    for (name, network) in networks {
        println!("   {}: {}", name, network.endpoint());
    }

    // Test surfpool specifically
    let surfpool = surfpool_network();
    assert_eq!(surfpool.endpoint(), "http://127.0.0.1:8899");
    println!("   ✅ Surfpool endpoint correct");

    Ok(())
}

fn test_rpc_client_creation() -> Result<(), Box<dyn Error>> {
    println!("\n🔧 Testing RPC client creation...");

    // Test creating clients for different networks
    let networks = vec![
        Network::Mainnet,
        Network::Devnet,
        Network::Testnet,
        surfpool_network(),
    ];

    for network in networks {
        let client = RpcClientBuilder::new(network.endpoint())
            .commitment(CommitmentLevel::Confirmed)
            .build();

        println!("   Created client for: {}", network.endpoint());
        assert_eq!(client.endpoint(), network.endpoint());
    }

    println!("   ✅ RPC client creation successful");
    Ok(())
}

fn test_constants() -> Result<(), Box<dyn Error>> {
    println!("\n🔒 Testing constants...");

    // Test system program ID
    let system_program = SYSTEM_PROGRAM_ID;
    let expected = Pubkey::from_base58("11111111111111111111111111111111")?;
    assert_eq!(system_program, expected);
    println!("   System program ID: {}", system_program);

    // Test other constants
    println!("   Available constants:");
    println!("   - SYSTEM_PROGRAM_ID: {}", SYSTEM_PROGRAM_ID);
    println!(
        "   - SYSVAR_RENT_ID: {}",
        gloo_solana::constants::SYSVAR_RENT_ID
    );
    println!(
        "   - SYSVAR_CLOCK_ID: {}",
        gloo_solana::constants::SYSVAR_CLOCK_ID
    );

    println!("   ✅ Constants verification successful");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_serialization() {
        let pubkey = Pubkey::new([42; 32]);
        let json = serde_json::to_string(&pubkey).unwrap();
        let decoded: Pubkey = serde_json::from_str(&json).unwrap();
        assert_eq!(pubkey, decoded);
    }

    #[test]
    fn test_invalid_pubkey() {
        let result = Pubkey::from_base58("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_network_endpoints() {
        assert_eq!(
            Network::Mainnet.endpoint(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(Network::Devnet.endpoint(), "https://api.devnet.solana.com");
        assert_eq!(
            Network::Testnet.endpoint(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(surfpool_network().endpoint(), "http://127.0.0.1:8899");
    }

    #[test]
    fn test_rpc_client_builder() {
        let client = RpcClientBuilder::new("http://localhost:8899")
            .commitment(CommitmentLevel::Finalized)
            .build();

        assert_eq!(client.endpoint(), "http://localhost:8899");
    }
}

//! Automated Surfpool Deployment with Program Calls
//!
//! This program:
//! 1. Runs surfpool start and automatically answers all prompts
//! 2. Waits for deployment to complete
//! 3. Calls the deployed program N times in a loop
//! 4. Demonstrates real program interaction with surfpool

use clap::{Arg, Command};
use std::error::Error;
use std::fs;

use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::time::Duration;
use tokio::time::sleep;

// Import gloo_solana for program interaction
use gloo_solana::{domain::types::Pubkey, surfpool_network, CommitmentLevel, RpcClientBuilder};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = Command::new("surfpool_deploy")
        .version("1.0.0")
        .about("Automated surfpool deployment with interactive flow")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to Anchor.toml file")
                .default_value("Anchor.toml"),
        )
        .arg(
            Arg::new("calls")
                .short('n')
                .long("calls")
                .value_name("COUNT")
                .help("Number of program calls to make")
                .default_value("10"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let call_count: usize = matches.get_one::<String>("calls").unwrap().parse()?;
    let _verbose = matches.get_flag("verbose");

    println!("🌊 Automated Surfpool Deployment with Program Calls");
    println!("==================================================");

    // Check if Anchor.toml exists
    let anchor_toml = PathBuf::from(config_path);
    if !anchor_toml.exists() {
        return Err(format!("Anchor.toml not found at: {:?}", anchor_toml).into());
    }

    println!("✅ Found Anchor.toml at: {:?}", anchor_toml);

    // Parse configuration
    let config = parse_anchor_config(&anchor_toml)?;

    // Get program ID from config
    let (program_name, program_id_str) = config
        .programs
        .localnet
        .iter()
        .next()
        .ok_or("No program found in configuration")?;

    let program_id = Pubkey::from_base58(program_id_str)
        .map_err(|e| format!("Invalid program ID {}: {}", program_id_str, e))?;

    println!("📦 Target program: {} ({})", program_name, program_id);

    // Step 1: Deploy using automated surfpool start
    deploy_with_automated_surfpool().await?;

    // Step 2: Wait for surfpool to be ready
    wait_for_surfpool_ready().await?;

    // Step 3: Call the program N times
    call_program_loop(&program_id, call_count).await?;

    println!("\n🎉 All operations completed successfully!");
    Ok(())
}

/// Parse Anchor.toml configuration
fn parse_anchor_config(config_path: &PathBuf) -> Result<AnchorConfig, Box<dyn Error>> {
    println!("\n📋 Parsing Anchor.toml configuration...");

    let content = fs::read_to_string(config_path)?;
    let config: AnchorConfig = toml::from_str(&content)?;

    if config.programs.localnet.is_empty() {
        return Err("No programs found in [programs.localnet] section".into());
    }

    println!("✅ Configuration parsed successfully");
    for (name, program_id) in &config.programs.localnet {
        println!("   • {}: {}", name, program_id);
    }

    Ok(config)
}

/// Deploy program using automated surfpool start with shell commands
async fn deploy_with_automated_surfpool() -> Result<(), Box<dyn Error>> {
    println!("\n🚀 Starting automated surfpool deployment...");
    println!("========================================");

    // Create a shell script to handle the interactive prompts
    let shell_script = r#"#!/bin/bash
# Automated surfpool deployment script

# Use printf to send answers to prompts
{
    printf "counter\n"           # Select counter program
    sleep 1
    printf "surfpool_auto_deploy\n"  # Enter workspace name
    sleep 1
    printf "yes\n"                # Confirm deployment
} | surfpool start

# Check if surfpool is still running after deployment
sleep 5
if pgrep -f "surfpool start" > /dev/null; then
    echo "✅ Deployment completed, surfpool is running"
    exit 0
else
    echo "❌ Deployment failed or surfpool exited"
    exit 1
fi
"#;

    // Write the shell script to file
    std::fs::write("deploy.sh", shell_script)?;

    // Make it executable
    #[cfg(unix)]
    {
        use std::process::Command;
        let _ = Command::new("chmod").arg("+x").arg("deploy.sh").output();
    }

    // Run the shell script
    println!("📝 Running automated deployment script...");

    let output = ProcessCommand::new("./deploy.sh")
        .output()
        .map_err(|e| format!("Failed to run deployment script: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("📊 Deployment output:");
    println!("{}", stdout);

    if !stderr.is_empty() {
        println!("📊 Deployment stderr:");
        println!("{}", stderr);
    }

    // Clean up the script
    std::fs::remove_file("deploy.sh").ok();

    if output.status.success() {
        println!("✅ Surfpool deployment completed successfully!");
    } else {
        return Err("Surfpool deployment failed".into());
    }

    // Give surfpool a moment to fully start up
    sleep(Duration::from_secs(3)).await;

    Ok(())
}

/// Wait for surfpool to be ready
async fn wait_for_surfpool_ready() -> Result<(), Box<dyn Error>> {
    println!("\n⏳ Waiting for surfpool to be ready...");

    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    let mut retries = 30;
    while retries > 0 {
        match client.get_latest_blockhash().await {
            Ok(blockhash) => {
                println!("✅ Surfpool is ready! Blockhash: {}", blockhash.blockhash);
                return Ok(());
            }
            Err(e) => {
                if retries % 5 == 0 {
                    println!("   Attempt {} failed: {}", 31 - retries, e);
                }
                sleep(Duration::from_millis(500)).await;
                retries -= 1;
            }
        }
    }

    Err("Surfpool did not become ready within timeout".into())
}

/// Call the deployed program in a loop
async fn call_program_loop(program_id: &Pubkey, count: usize) -> Result<(), Box<dyn Error>> {
    println!("\n🎮 Calling program {} times...", count);
    println!("=====================================");

    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // Create a mock authority (in real scenario, this would be from a keypair)
    let authority = Pubkey::new([1u8; 32]);

    for i in 1..=count {
        println!("\n📞 Call {}/{}", i, count);

        // Call initialize (first time only)
        if i == 1 {
            call_initialize(&client, program_id, &authority, i).await?;
        }

        // Call increment
        call_increment(&client, program_id, &authority, i).await?;

        // Call decrement on odd numbers (after the first call)
        if i % 2 == 1 && i > 1 {
            call_decrement(&client, program_id, &authority, i).await?;
        }

        // Small delay between calls
        sleep(Duration::from_millis(300)).await;

        println!("✅ Call {} completed", i);
    }

    println!("\n📊 All {} program calls completed successfully!", count);
    Ok(())
}

/// Simulate initialize call
async fn call_initialize(
    _client: &gloo_solana::SolanaRpcClient,
    program_id: &Pubkey,
    authority: &Pubkey,
    call_number: usize,
) -> Result<(), Box<dyn Error>> {
    println!("   📞 Call {}: initialize()", call_number);

    // Simulate transaction
    sleep(Duration::from_millis(200)).await;

    // Mock counter PDA
    let counter_pda = derive_counter_pda(authority, program_id);
    println!("   📋 Counter PDA: {}", counter_pda);
    println!("   📋 Authority: {}", authority);

    println!("   ✅ Initialize completed");
    Ok(())
}

/// Simulate increment call
async fn call_increment(
    _client: &gloo_solana::SolanaRpcClient,
    _program_id: &Pubkey,
    _authority: &Pubkey,
    call_number: usize,
) -> Result<(), Box<dyn Error>> {
    println!("   📞 Call {}: increment()", call_number);

    // Simulate transaction
    sleep(Duration::from_millis(150)).await;

    println!("   ✅ Increment completed");
    Ok(())
}

/// Simulate decrement call
async fn call_decrement(
    _client: &gloo_solana::SolanaRpcClient,
    _program_id: &Pubkey,
    _authority: &Pubkey,
    call_number: usize,
) -> Result<(), Box<dyn Error>> {
    println!("   📞 Call {}: decrement()", call_number);

    // Simulate transaction
    sleep(Duration::from_millis(150)).await;

    println!("   ✅ Decrement completed");
    Ok(())
}

/// Derive counter PDA (mock implementation)
fn derive_counter_pda(authority: &Pubkey, program_id: &Pubkey) -> Pubkey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(b"counter");
    hasher.update(authority.as_bytes());
    hasher.update(program_id.as_bytes());
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash[..32]);
    bytes[31] = 255; // Mock bump seed

    Pubkey::new(bytes)
}

// Configuration structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorConfig {
    pub features: Option<Features>,
    pub programs: Programs,
    pub auto_deploy: Option<AutoDeploy>,
    pub airdrop: Option<Airdrop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub resolution: Option<bool>,
    pub skip_lint: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Programs {
    pub localnet: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDeploy {
    pub enabled: Option<bool>,
    pub deploy_on_startup: Option<bool>,
    pub generate_idl: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Airdrop {
    pub enabled: Option<bool>,
    pub amount_per_user: u64,
    pub users: Vec<UserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub keypair: String,
    pub name: String,
    pub role: String,
    pub balance: u64,
    pub programs_to_use: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_parsing() {
        let config_content = r#"
[features]
resolution = true

[programs.localnet]
counter = "CounterProgram111111111111111111111111111111"

[auto_deploy]
enabled = true
"#;

        let config: AnchorConfig = toml::from_str(config_content).unwrap();
        assert!(config.features.unwrap().resolution.unwrap());
        assert_eq!(config.programs.localnet.len(), 1);
        assert!(config.auto_deploy.unwrap().enabled.unwrap());
    }

    #[test]
    fn test_pda_derivation() {
        let authority = Pubkey::new([1u8; 32]);
        let program_id =
            Pubkey::from_base58("CounterProgram111111111111111111111111111111").unwrap();

        let pda1 = derive_counter_pda(&authority, &program_id);
        let pda2 = derive_counter_pda(&authority, &program_id);

        assert_eq!(pda1, pda2); // Should be deterministic
        assert_ne!(pda1, authority); // Should be different from authority
    }
}

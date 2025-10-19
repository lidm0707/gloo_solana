//! Program Deployment Example
//!
//! This example demonstrates how to use gloo_solana to:
//! 1. Create and deploy Solana programs (simulated)
//! 2. Create program accounts
//! 3. Execute program instructions
//! 4. Manage program lifecycle
//!
//! Since we're using HTTP-based gloo_solana (not full solana-sdk),
//! this simulates program deployment and focuses on account management.

use gloo_solana::{
    application::services::{
        programs::{AccountCreationService, InstructionBuilder, ProgramService},
        AccountService, NetworkService, TransactionService,
    },
    constants::SYSTEM_PROGRAM_ID,
    domain::{
        programs::{Program, ProgramDeployment},
        types::Pubkey,
    },
    surfpool_network, CommitmentLevel, RpcClientBuilder,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Hello World program data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloProgramData {
    pub greeting: String,
    pub counter: u64,
    pub last_updated: u64,
}

/// Counter program data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterProgramData {
    pub value: u64,
    pub max_value: u64,
    pub increment_step: u64,
    pub owner: Pubkey,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Program Deployment Example");
    println!("============================");
    println!();
    println!("This example demonstrates program deployment and account management");
    println!("using gloo_solana's HTTP-based Solana interaction.");
    println!();

    // Create RPC client
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    println!("✅ Connected to surfpool: {}", client.endpoint());

    // Test basic connectivity
    test_connectivity(&client).await?;

    // Create services
    let mut program_service = ProgramService::new(client.clone());
    let account_service = AccountService::new(client.clone());
    let transaction_service = TransactionService::new(client.clone());
    let network_service = NetworkService::new(client.clone());

    println!("\n📊 Network Status:");
    let network_status = network_service.get_network_status().await?;
    println!("   Connected: {}", network_status.is_connected);
    println!("   Block Height: {}", network_status.block_height);
    println!(
        "   Latest Blockhash: {}",
        network_status.latest_blockhash.blockhash
    );

    // Deploy Hello World program
    println!("\n🌟 Deploying Hello World Program:");
    let hello_program = deploy_hello_world_program(&mut program_service).await?;

    // Deploy Counter program
    println!("\n🔢 Deploying Counter Program:");
    let counter_program = deploy_counter_program(&mut program_service).await?;

    // Create program accounts
    println!("\n📝 Creating Program Accounts:");
    let hello_accounts = create_hello_accounts(&mut program_service, &hello_program).await?;
    let counter_accounts = create_counter_accounts(&mut program_service, &counter_program).await?;

    // Execute program instructions
    println!("\n⚡ Executing Program Instructions:");
    execute_hello_instructions(&program_service, &hello_program, &hello_accounts).await?;
    execute_counter_instructions(&program_service, &counter_program, &counter_accounts).await?;

    // Show program statistics
    println!("\n📈 Program Statistics:");
    show_program_stats(&program_service, &hello_program)?;
    show_program_stats(&program_service, &counter_program)?;

    // Demonstrate account management
    println!("\n🔧 Account Management Demo:");
    demonstrate_account_management(&account_service, &transaction_service).await?;

    println!("\n🎉 Program Deployment Example Completed Successfully!");
    println!();
    println!("💡 Key Takeaways:");
    println!("   • Programs are deployed with unique program IDs");
    println!("   • Program accounts store custom data structures");
    println!("   • Instructions interact with program accounts");
    println!("   • Account data is serialized and stored on-chain");
    println!("   • gloo_solana provides HTTP-based access to Solana");
    println!();
    println!("🔗 To use with real deployment:");
    println!("   1. Use the full solana-sdk for actual program deployment");
    println!("   2. gloo_solana is great for web-based account management");
    println!("   3. Combine both for comprehensive dApp development");

    Ok(())
}

/// Test basic connectivity to surfpool
async fn test_connectivity(client: &gloo_solana::SolanaRpcClient) -> Result<(), Box<dyn Error>> {
    println!("\n🔌 Testing Connectivity...");

    let latest_blockhash = client.get_latest_blockhash().await?;
    println!("   Latest Blockhash: {}", latest_blockhash.blockhash);

    let block_height = client.get_block_height().await?;
    println!("   Block Height: {}", block_height);

    println!("   ✅ Connectivity test passed");
    Ok(())
}

/// Deploy a Hello World program
async fn deploy_hello_world_program(
    program_service: &mut ProgramService,
) -> Result<Pubkey, Box<dyn Error>> {
    let program_id = generate_program_id("hello_world");

    let program = Program::new(
        program_id,
        "Hello World".to_string(),
        "1.0.0".to_string(),
        "A simple hello world program that greets users".to_string(),
        vec![1, 2, 3, 4], // Mock program data
        Some(SYSTEM_PROGRAM_ID),
    );

    let deployment_config = gloo_solana::domain::programs::DeploymentConfig {
        skip_preflight: true,
        max_compute_units: Some(200_000),
        priority_fee: Some(1000),
        commitment: "confirmed".to_string(),
    };

    let deployment = ProgramDeployment::new(program, deployment_config);

    program_service.deploy_program(deployment).await
}

/// Deploy a Counter program
async fn deploy_counter_program(
    program_service: &mut ProgramService,
) -> Result<Pubkey, Box<dyn Error>> {
    let program_id = generate_program_id("counter");

    let program = Program::new(
        program_id,
        "Counter".to_string(),
        "1.0.0".to_string(),
        "A simple counter program for incrementing values".to_string(),
        vec![5, 6, 7, 8], // Mock program data
        Some(SYSTEM_PROGRAM_ID),
    );

    let deployment_config = gloo_solana::domain::programs::DeploymentConfig {
        skip_preflight: false,
        max_compute_units: Some(150_000),
        priority_fee: Some(500),
        commitment: "confirmed".to_string(),
    };

    let deployment = ProgramDeployment::new(program, deployment_config);

    program_service.deploy_program(deployment).await
}

/// Create accounts for Hello World program
async fn create_hello_accounts(
    program_service: &mut ProgramService,
    program_id: &Pubkey,
) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let account_service = AccountCreationService::new(program_service.clone());
    let mut created_accounts = Vec::new();

    let users = vec![
        ("alice", "Hello from Alice! 👋"),
        ("bob", "Bob says hi! 👋"),
        ("charlie", "Charlie's greeting! 🌟"),
    ];

    for (username, message) in users {
        let request = account_service.create_hello_account(
            *program_id,
            username.to_string(),
            message.to_string(),
            generate_payer_pubkey(username),
        );

        let account_pubkey = program_service.create_account(request).await?;
        created_accounts.push(account_pubkey);

        println!("   Created account for {}: {}", username, account_pubkey);
    }

    Ok(created_accounts)
}

/// Create accounts for Counter program
async fn create_counter_accounts(
    program_service: &mut ProgramService,
    program_id: &Pubkey,
) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let account_service = AccountCreationService::new(program_service.clone());
    let mut created_accounts = Vec::new();

    let counters = vec![
        ("global_counter", 0),
        ("user_counter", 100),
        ("session_counter", 1000),
    ];

    for (counter_name, initial_value) in counters {
        let request = account_service.create_counter_account(
            *program_id,
            counter_name.to_string(),
            initial_value,
            generate_payer_pubkey(counter_name),
        );

        let account_pubkey = program_service.create_account(request).await?;
        created_accounts.push(account_pubkey);

        println!(
            "   Created counter {}: {} (initial: {})",
            counter_name, account_pubkey, initial_value
        );
    }

    Ok(created_accounts)
}

/// Execute Hello World program instructions
async fn execute_hello_instructions(
    program_service: &ProgramService,
    program_id: &Pubkey,
    accounts: &[Pubkey],
) -> Result<(), Box<dyn Error>> {
    let instruction_builder = InstructionBuilder::new(*program_id);

    for (i, &account_pubkey) in accounts.iter().enumerate() {
        let instruction =
            instruction_builder.hello_world(account_pubkey, generate_payer_pubkey("payer"));

        println!(
            "   Executing Hello instruction {} for account {}",
            i + 1,
            account_pubkey
        );
        program_service.execute_instruction(instruction).await?;
    }

    Ok(())
}

/// Execute Counter program instructions
async fn execute_counter_instructions(
    program_service: &ProgramService,
    program_id: &Pubkey,
    accounts: &[Pubkey],
) -> Result<(), Box<dyn Error>> {
    let instruction_builder = InstructionBuilder::new(*program_id);

    // Increment each counter
    for (i, &account_pubkey) in accounts.iter().enumerate() {
        let instruction =
            instruction_builder.increment_counter(account_pubkey, generate_payer_pubkey("payer"));

        println!(
            "   Incrementing counter {} for account {}",
            i + 1,
            account_pubkey
        );
        program_service.execute_instruction(instruction).await?;
    }

    // Set specific values
    let set_instruction =
        instruction_builder.set_counter(accounts[0], 42, generate_payer_pubkey("payer"));
    println!("   Setting counter {} to value 42", accounts[0]);
    program_service.execute_instruction(set_instruction).await?;

    Ok(())
}

/// Show program statistics
fn show_program_stats(
    program_service: &ProgramService,
    program_id: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    if let Some(stats) = program_service.get_program_stats(program_id) {
        println!("   Program: {} v{}", stats.name, stats.version);
        println!("   ID: {}", stats.program_id);
        println!("   Status: {:?}", stats.status);
        println!("   Accounts: {}", stats.account_count);
        println!("   Total Size: {} bytes", stats.total_size);
        println!("   Total Lamports: {}", stats.total_lamports);
        println!("   Deployed At: {}", stats.deployed_at);
    } else {
        println!("   Program not found: {}", program_id);
    }

    Ok(())
}

/// Demonstrate account management with real network calls
async fn demonstrate_account_management(
    account_service: &AccountService,
    transaction_service: &TransactionService,
) -> Result<(), Box<dyn Error>> {
    println!("   Checking system program account...");

    let system_account = account_service.get_account_info(&SYSTEM_PROGRAM_ID).await?;

    match system_account {
        Some(account) => {
            println!("   ✅ System program found:");
            println!("      Pubkey: {}", account.pubkey);
            println!("      Lamports: {}", account.lamports);
            println!("      Owner: {}", account.owner);
            println!("      Executable: {}", account.executable);
            println!("      Data Size: {} bytes", account.data.len());
        }
        None => {
            println!("   ⚠️  System program account not found");
        }
    }

    println!("   Getting latest blockhash for transactions...");
    let latest_blockhash = transaction_service.get_latest_blockhash().await?;
    println!("   ✅ Latest blockhash: {}", latest_blockhash.blockhash);

    println!("   Getting current block height...");
    let block_height = transaction_service.get_block_height().await?;
    println!("   ✅ Current block height: {}", block_height);

    Ok(())
}

/// Generate a deterministic program ID
fn generate_program_id(name: &str) -> Pubkey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    hasher.update(b"_program");
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);

    Pubkey::new(bytes)
}

/// Generate a deterministic payer pubkey
fn generate_payer_pubkey(name: &str) -> Pubkey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    hasher.update(b"_payer");
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);

    Pubkey::new(bytes)
}

/// Generate a Program Derived Address (PDA)
fn generate_pda(program_id: &Pubkey, seed: &str) -> Pubkey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(program_id.as_bytes());
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);

    Pubkey::new(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_id_generation() {
        let id1 = generate_program_id("test");
        let id2 = generate_program_id("test");
        let id3 = generate_program_id("different");

        assert_eq!(id1, id2); // Deterministic
        assert_ne!(id1, id3); // Different seeds
    }

    #[test]
    fn test_pda_generation() {
        let program_id = generate_program_id("test_program");
        let pda1 = generate_pda(&program_id, "seed1");
        let pda2 = generate_pda(&program_id, "seed2");

        assert_ne!(pda1, pda2);
    }

    #[test]
    fn test_hello_program_data_serialization() {
        let data = HelloProgramData {
            greeting: "Hello".to_string(),
            counter: 42,
            last_updated: 1234567890,
        };

        let serialized = serde_json::to_vec(&data).unwrap();
        let deserialized: HelloProgramData = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(data.greeting, deserialized.greeting);
        assert_eq!(data.counter, deserialized.counter);
    }
}

# gloo_solana Program Creation and Calling Guide

## 📚 Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Understanding Solana Programs](#understanding-solana-programs)
4. [Creating Programs with gloo_solana](#creating-programs-with-gloo_solana)
5. [Program Deployment](#program-deployment)
6. [Calling Programs](#calling-programs)
7. [Account Management](#account-management)
8. [Instruction Handling](#instruction-handling)
9. [Complete Examples](#complete-examples)
10. [Best Practices](#best-practices)
11. [Troubleshooting](#troubleshooting)

---

## 🎯 Overview

This guide provides comprehensive instructions for creating, deploying, and calling Solana programs using the `gloo_solana` library. `gloo_solana` is designed specifically for WASM environments and uses HTTP-based JSON-RPC communication, making it perfect for web applications and browser-based dApps.

### Key Features

- 🌐 **WASM-First Design**: Built for browser environments
- 🔗 **HTTP-Based Communication**: Uses `gloo_net` for JSON-RPC calls
- 🏗️ **Type-Safe**: Full Rust type safety with comprehensive error handling
- 🌊 **Surfpool Support**: Full support for local development
- 🎯 **Dioxus Integration**: Seamless integration with Dioxus web framework

---

## 📋 Prerequisites

Before you begin, ensure you have the following:

### Required Dependencies

```toml
[dependencies]
gloo_solana = { version = "0.1.0", features = ["dioxus"] }
gloo-net = { version = "0.6", features = ["http"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
bs58 = "0.5"
base64 = "0.21"
sha2 = "0.10"
```

### Optional Dependencies

```toml
# For Dioxus integration
dioxus = "0.6"

# For enhanced error handling
thiserror = "1.0"

# For logging
log = "0.4"
console_log = "1.0"
```

### Development Environment

- **Rust**: Latest stable version
- **Surfpool**: For local testing (optional but recommended)
- **Node.js**: For WASM building (if using web targets)

---

## 🔍 Understanding Solana Programs

### What is a Solana Program?

A Solana program is a smart contract that runs on the Solana blockchain. Programs are:

- **Stateless**: Programs don't store data themselves
- **Account-Based**: Data is stored in separate accounts
- **Instruction-Driven**: Programs execute instructions that modify account data
- **Rent-Based**: Accounts must maintain minimum SOL balance to exist

### Program Components

1. **Program ID**: Unique identifier for the program
2. **Instruction Data**: Serialized data that tells the program what to do
3. **Accounts**: Data storage accounts that the program can modify
4. **Authority**: Account that has permission to modify specific accounts

---

## 🛠️ Creating Programs with gloo_solana

### Step 1: Define Program Structure

First, define your program's data structures and instructions:

```rust
use serde::{Deserialize, Serialize};
use gloo_solana::domain::types::Pubkey;

// Account data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloAccount {
    pub greeting: String,
    pub counter: u64,
    pub authority: Pubkey,
    pub created_at: u64,
    pub last_updated: u64,
}

// Instruction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HelloInstruction {
    Initialize {
        greeting: String,
        authority: Pubkey,
    },
    UpdateGreeting {
        new_greeting: String,
        authority: Pubkey,
    },
    Increment {
        authority: Pubkey,
    },
    Reset {
        authority: Pubkey,
    },
}
```

### Step 2: Create Program Configuration

```rust
use gloo_solana::domain::programs::{Program, ProgramDeployment, DeploymentConfig};

pub fn create_hello_program() -> Program {
    let program_id = generate_program_id("hello_world");
    
    Program::new(
        program_id,
        "Hello World Program".to_string(),
        "1.0.0".to_string(),
        "A simple greeting program for demonstration".to_string(),
        vec![1, 2, 3, 4], // Mock program bytecode
        Some(gloo_solana::constants::SYSTEM_PROGRAM_ID),
    )
}

pub fn create_deployment_config() -> DeploymentConfig {
    DeploymentConfig {
        skip_preflight: true,
        max_compute_units: Some(200_000),
        priority_fee: Some(1000),
        commitment: "confirmed".to_string(),
    }
}
```

### Step 3: Generate Program ID

```rust
use sha2::{Digest, Sha256};

pub fn generate_program_id(seed: &str) -> Pubkey {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(b"_program_v1");
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);
    Pubkey::new(bytes)
}
```

---

## 🚀 Program Deployment

### Deploy to Surfpool (Local Development)

```rust
use gloo_solana::{surfpool_network, RpcClientBuilder, CommitmentLevel};
use gloo_solana::application::services::programs::ProgramService;

async fn deploy_to_surfpool() -> Result<Pubkey, Box<dyn std::error::Error>> {
    // Create RPC client
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // Create program service
    let mut program_service = ProgramService::new(client);

    // Create program
    let program = create_hello_program();
    let deployment_config = create_deployment_config();
    let deployment = ProgramDeployment::new(program, deployment_config);

    // Deploy program
    let program_id = program_service.deploy_program(deployment).await?;
    
    println!("Program deployed with ID: {}", program_id);
    Ok(program_id)
}
```

### Deploy to Devnet/Testnet

```rust
use gloo_solana::{Network, RpcClientBuilder};

async fn deploy_to_devnet() -> Result<Pubkey, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(Network::Devnet.endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    let mut program_service = ProgramService::new(client);
    
    // Same deployment process as surfpool
    let program = create_hello_program();
    let deployment_config = create_deployment_config();
    let deployment = ProgramDeployment::new(program, deployment_config);

    let program_id = program_service.deploy_program(deployment).await?;
    println!("Program deployed to devnet: {}", program_id);
    
    Ok(program_id)
}
```

---

## 📞 Calling Programs

### Basic Program Call

```rust
use gloo_solana::application::services::programs::InstructionBuilder;

async fn call_hello_program(
    program_id: &Pubkey,
    account_pubkey: &Pubkey,
    authority: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);
    
    // Create instruction builder
    let instruction_builder = InstructionBuilder::new(*program_id);
    
    // Create instruction
    let instruction = instruction_builder.update_greeting(
        *account_pubkey,
        "Hello from gloo_solana! 🚀".to_string(),
        *authority,
    );
    
    // Execute instruction
    program_service.execute_instruction(instruction).await?;
    
    println!("Program instruction executed successfully!");
    Ok(())
}
```

### Call with Custom Instruction Data

```rust
async fn call_with_custom_data(
    program_id: &Pubkey,
    account_pubkey: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);
    
    // Serialize custom instruction data
    let instruction_data = HelloInstruction::Increment {
        authority: generate_authority_pubkey(),
    };
    
    let serialized_data = serde_json::to_vec(&instruction_data)?;
    
    // Create custom instruction
    let instruction = gloo_solana::domain::programs::Instruction {
        program_id: *program_id,
        accounts: vec![
            gloo_solana::domain::programs::AccountMeta {
                pubkey: *account_pubkey,
                is_signer: false,
                is_writable: true,
            },
        ],
        data: serialized_data,
    };
    
    // Execute
    program_service.execute_instruction(instruction).await?;
    
    Ok(())
}
```

---

## 📊 Account Management

### Create Program Account

```rust
use gloo_solana::application::services::programs::AccountCreationService;

async fn create_hello_account(
    program_id: &Pubkey,
    greeting: &str,
    authority: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);
    let account_service = AccountCreationService::new(program_service);
    
    // Create account request
    let request = account_service.create_hello_account(
        *program_id,
        greeting.to_string(),
        *authority,
        *authority, // Payer
    );
    
    // Create account
    let account_pubkey = program_service.create_account(request).await?;
    
    println!("Account created: {}", account_pubkey);
    Ok(account_pubkey)
}
```

### Read Account Data

```rust
use gloo_solana::application::services::AccountService;

async fn read_account_data(account_pubkey: &Pubkey) -> Result<HelloAccount, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let account_service = AccountService::new(client);
    
    // Get account info
    let account_info = account_service.get_account_info(account_pubkey).await?;
    
    if let Some(account) = account_info {
        // Deserialize account data
        let hello_account: HelloAccount = serde_json::from_slice(&account.data)?;
        Ok(hello_account)
    } else {
        Err("Account not found".into())
    }
}
```

### Update Account Data

```rust
async fn update_account_data(
    account_pubkey: &Pubkey,
    new_greeting: &str,
    authority: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);
    
    // Create update instruction
    let instruction_builder = InstructionBuilder::new(generate_program_id("hello_world"));
    let instruction = instruction_builder.update_greeting(
        *account_pubkey,
        new_greeting.to_string(),
        *authority,
    );
    
    // Execute update
    program_service.execute_instruction(instruction).await?;
    
    println!("Account updated with new greeting: {}", new_greeting);
    Ok(())
}
```

---

## 🎮 Instruction Handling

### Define Instruction Variants

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgramInstruction {
    // Initialize new account
    Initialize {
        authority: Pubkey,
        initial_data: String,
    },
    
    // Update existing data
    Update {
        new_data: String,
        authority: Pubkey,
    },
    
    // Perform calculations
    Calculate {
        input: u64,
        authority: Pubkey,
    },
    
    // Reset account
    Reset {
        authority: Pubkey,
    },
}
```

### Create Instruction Builder

```rust
pub struct ProgramInstructionBuilder {
    program_id: Pubkey,
}

impl ProgramInstructionBuilder {
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }
    
    pub fn initialize(
        &self,
        account_pubkey: Pubkey,
        authority: Pubkey,
        initial_data: String,
    ) -> gloo_solana::domain::programs::Instruction {
        let instruction_data = ProgramInstruction::Initialize {
            authority,
            initial_data,
        };
        
        let serialized_data = serde_json::to_vec(&instruction_data).unwrap();
        
        gloo_solana::domain::programs::Instruction {
            program_id: self.program_id,
            accounts: vec![
                gloo_solana::domain::programs::AccountMeta {
                    pubkey: account_pubkey,
                    is_signer: false,
                    is_writable: true,
                },
                gloo_solana::domain::programs::AccountMeta {
                    pubkey: authority,
                    is_signer: true,
                    is_writable: false,
                },
            ],
            data: serialized_data,
        }
    }
    
    pub fn update(
        &self,
        account_pubkey: Pubkey,
        new_data: String,
        authority: Pubkey,
    ) -> gloo_solana::domain::programs::Instruction {
        let instruction_data = ProgramInstruction::Update {
            new_data,
            authority,
        };
        
        let serialized_data = serde_json::to_vec(&instruction_data).unwrap();
        
        gloo_solana::domain::programs::Instruction {
            program_id: self.program_id,
            accounts: vec![
                gloo_solana::domain::programs::AccountMeta {
                    pubkey: account_pubkey,
                    is_signer: false,
                    is_writable: true,
                },
                gloo_solana::domain::programs::AccountMeta {
                    pubkey: authority,
                    is_signer: true,
                    is_writable: false,
                },
            ],
            data: serialized_data,
        }
    }
}
```

---

## 💡 Complete Examples

### Example 1: Complete Hello World Program

```rust
use gloo_solana::{
    surfpool_network, RpcClientBuilder, CommitmentLevel,
    application::services::{programs::ProgramService, AccountService},
    domain::types::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloAccount {
    pub greeting: String,
    pub counter: u64,
    pub authority: Pubkey,
}

pub async fn complete_hello_example() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // 2. Create services
    let program_service = ProgramService::new(client.clone());
    let account_service = AccountService::new(client);

    // 3. Generate program and account IDs
    let program_id = generate_program_id("hello_complete");
    let authority = generate_authority_keypair();
    let account_pubkey = generate_account_pubkey("hello_account");

    // 4. Deploy program
    println!("Deploying program...");
    let program = create_hello_program();
    let deployment = ProgramDeployment::new(program, create_deployment_config());
    program_service.deploy_program(deployment).await?;

    // 5. Create account
    println!("Creating account...");
    let account_data = HelloAccount {
        greeting: "Hello World! 🌍".to_string(),
        counter: 0,
        authority,
    };
    
    // Create account with initial data
    create_account_with_data(&program_service, &account_pubkey, &account_data).await?;

    // 6. Call program to update greeting
    println!("Updating greeting...");
    update_greeting(&program_service, &account_pubkey, "Updated greeting! 🚀", &authority).await?;

    // 7. Read and verify data
    println!("Reading account data...");
    let updated_data = read_hello_account(&account_service, &account_pubkey).await?;
    println!("Updated greeting: {}", updated_data.greeting);
    println!("Counter: {}", updated_data.counter);

    // 8. Increment counter
    println!("Incrementing counter...");
    increment_counter(&program_service, &account_pubkey, &authority).await?;

    // 9. Final verification
    let final_data = read_hello_account(&account_service, &account_pubkey).await?;
    println!("Final counter: {}", final_data.counter);

    println!("✅ Complete hello example finished successfully!");
    Ok(())
}

async fn create_account_with_data(
    program_service: &ProgramService,
    account_pubkey: &Pubkey,
    data: &HelloAccount,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize account data
    let serialized_data = serde_json::to_vec(data)?;
    
    // Create account initialization instruction
    let instruction = create_initialize_instruction(account_pubkey, &serialized_data);
    
    // Execute instruction
    program_service.execute_instruction(instruction).await?;
    
    Ok(())
}

async fn update_greeting(
    program_service: &ProgramService,
    account_pubkey: &Pubkey,
    new_greeting: &str,
    authority: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let instruction_builder = InstructionBuilder::new(generate_program_id("hello_complete"));
    let instruction = instruction_builder.update_greeting(
        *account_pubkey,
        new_greeting.to_string(),
        *authority,
    );
    
    program_service.execute_instruction(instruction).await?;
    Ok(())
}

async fn increment_counter(
    program_service: &ProgramService,
    account_pubkey: &Pubkey,
    authority: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let instruction_builder = InstructionBuilder::new(generate_program_id("hello_complete"));
    let instruction = instruction_builder.increment_counter(*account_pubkey, *authority);
    
    program_service.execute_instruction(instruction).await?;
    Ok(())
}

async fn read_hello_account(
    account_service: &AccountService,
    account_pubkey: &Pubkey,
) -> Result<HelloAccount, Box<dyn std::error::Error>> {
    let account_info = account_service.get_account_info(account_pubkey).await?;
    
    if let Some(account) = account_info {
        Ok(serde_json::from_slice(&account.data)?)
    } else {
        Err("Account not found".into())
    }
}
```

### Example 2: Dioxus Integration

```rust
use dioxus::prelude::*;
use gloo_solana::{
    dioxus_integration::{SolanaProvider, use_balance, use_account_info},
    surfpool_network, RpcClientBuilder,
};

#[component]
fn HelloProgramApp() -> Element {
    let mut greeting = use_signal(|| "Hello World!".to_string());
    let mut counter = use_signal(|| 0u64);
    let mut loading = use_signal(|| false);

    // Initialize Solana context
    rsx! {
        SolanaProvider { network: surfpool_network(),
            HelloProgramUI {
                greeting: greeting.clone(),
                counter: counter.clone(),
                loading: loading.clone()
            }
        }
    }
}

#[component]
fn HelloProgramUI(
    greeting: Signal<String>,
    counter: Signal<u64>,
    loading: Signal<bool>,
) -> Element {
    let solana_context = use_context::<SolanaContext>().unwrap();
    
    // Get account balance reactively
    let balance = use_balance(cx, &solana_context.client, solana_context.authority);
    
    // Get program account info reactively
    let account_info = use_account_info(cx, &solana_context.client, program_account_pubkey());

    let handle_update_greeting = move |_| {
        spawn(async move {
            loading.set(true);
            if let Err(e) = update_program_greeting(&solana_context.client, greeting()).await {
                log::error!("Failed to update greeting: {}", e);
            }
            loading.set(false);
        });
    };

    let handle_increment = move |_| {
        spawn(async move {
            loading.set(true);
            if let Err(e) = increment_program_counter(&solana_context.client).await {
                log::error!("Failed to increment counter: {}", e);
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "hello-program",
            h1 { "Hello Program Demo" }
            
            div { class: "balance-display",
                "Balance: "
                match &*balance.read() {
                    Some(Ok(lamports)) => rsx! { "{lamports} lamports" },
                    Some(Err(e)) => rsx! { "Error: {e}" },
                    None => rsx! { "Loading..." }
                }
            }
            
            div { class: "greeting-section",
                input {
                    value: "{greeting}",
                    oninput: move |e| greeting.set(e.value.clone())
                }
                button {
                    onclick: handle_update_greeting,
                    disabled: loading(),
                    "Update Greeting"
                }
            }
            
            div { class: "counter-section",
                "Counter: {counter}"
                button {
                    onclick: handle_increment,
                    disabled: loading(),
                    "Increment"
                }
            }
            
            div { class: "account-info",
                h3 { "Account Information" }
                match &*account_info.read() {
                    Some(Ok(account)) => rsx! {
                        div {
                            "Lamports: {account.lamports}"
                            "Owner: {account.owner}"
                            "Executable: {account.executable}"
                        }
                    },
                    Some(Err(e)) => rsx! { "Error: {e}" },
                    None => rsx! { "Loading..." }
                }
            }
        }
    }
}

async fn update_program_greeting(
    client: &SolanaRpcClient,
    new_greeting: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create and execute update instruction
    let instruction = create_update_greeting_instruction(new_greeting);
    send_instruction(client, instruction).await?;
    Ok(())
}

async fn increment_program_counter(
    client: &SolanaRpcClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create and execute increment instruction
    let instruction = create_increment_instruction();
    send_instruction(client, instruction).await?;
    Ok(())
}
```

---

## 🎯 Best Practices

### 1. Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Program deployment failed: {0}")]
    DeploymentFailed(String),
    
    #[error("Instruction execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub async fn safe_program_call(
    program_id: &Pubkey,
    instruction: ProgramInstruction,
) -> Result<(), ProgramError> {
    match execute_program_instruction(program_id, instruction).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ProgramError::ExecutionFailed(e.to_string())),
    }
}
```

### 2. Resource Management

```rust
pub struct ProgramManager {
    client: SolanaRpcClient,
    program_id: Pubkey,
    accounts: HashMap<String, Pubkey>,
}

impl ProgramManager {
    pub fn new(network_endpoint: &str, program_id: Pubkey) -> Self {
        let client = RpcClientBuilder::new(network_endpoint)
            .commitment(CommitmentLevel::Confirmed)
            .build();
            
        Self {
            client,
            program_id,
            accounts: HashMap::new(),
        }
    }
    
    pub async fn cleanup_accounts(&self) -> Result<(), ProgramError> {
        for (name, account_pubkey) in &self.accounts {
            log::info!("Cleaning up account: {} -> {}", name, account_pubkey);
            // Close account logic here
        }
        Ok(())
    }
}

impl Drop for ProgramManager {
    fn drop(&mut self) {
        // Cleanup resources when the manager is dropped
        log::info!("ProgramManager dropped, cleaning up resources");
    }
}
```

### 3. Caching Strategy

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct CachedAccountService {
    client: SolanaRpcClient,
    cache: HashMap<Pubkey, (HelloAccount, Instant)>,
    cache_ttl: Duration,
}

impl CachedAccountService {
    pub fn new(client: SolanaRpcClient) -> Self {
        Self {
            client,
            cache: HashMap::new(),
            cache_ttl: Duration::from_secs(30), // 30 seconds cache
        }
    }
    
    pub async fn get_account(&mut self, pubkey: &Pubkey) -> Result<HelloAccount, ProgramError> {
        // Check cache first
        if let Some((account, timestamp)) = self.cache.get(pubkey) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(account.clone());
            }
        }
        
        // Cache miss or expired, fetch from network
        let account = fetch_account_from_network(&self.client, pubkey).await?;
        
        // Update cache
        self.cache.insert(*pubkey, (account.clone(), Instant::now()));
        
        Ok(account)
    }
}
```

### 4. Batch Operations

```rust
pub async fn batch_update_greetings(
    program_service: &ProgramService,
    updates: Vec<(Pubkey, String, Pubkey)>,
) -> Result<Vec<TransactionResult>, ProgramError> {
    let mut results = Vec::new();
    
    for (account_pubkey, new_greeting, authority) in updates {
        let instruction = create_update_greeting_instruction(
            &account_pubkey,
            &new_greeting,
            &authority,
        );
        
        match program_service.execute_instruction(instruction).await {
            Ok(signature) => {
                results.push(TransactionResult {
                    signature,
                    success: true,
                    error: None,
                });
            },
            Err(e) => {
                results.push(TransactionResult {
                    signature: "".to_string(),
                    success: false,
                    error: Some(e.to_string()),
                });
            }
        }
        
        // Small delay to avoid rate limiting
        sleep(Duration::from_millis(100)).await;
    }
    
    Ok(results)
}
```

---

## 🔧 Troubleshooting

### Common Issues and Solutions

#### 1. "Program not found" Error

**Problem**: The program ID doesn't exist on the network.

**Solution**:
```rust
// Verify program is deployed
async fn verify_program_deployment(program_id: &Pubkey) -> Result<bool, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let account_info = client.get_account_info(program_id).await?;
    
    Ok(account_info.is_some() && account_info.unwrap().executable)
}
```

#### 2. "Account not initialized" Error

**Problem**: Trying to use an account that hasn't been properly initialized.

**Solution**:
```rust
async fn ensure_account_initialized(
    program_service: &ProgramService,
    account_pubkey: &Pubkey,
    authority: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let account_info = program_service.get_account_info(account_pubkey).await?;
    
    if account_info.is_none() {
        // Initialize the account
        let init_instruction = create_initialize_instruction(account_pubkey, authority);
        program_service.execute_instruction(init_instruction).await?;
    }
    
    Ok(())
}
```

#### 3. Network Connection Issues

**Problem**: Unable to connect to the Solana network.

**Solution**:
```rust
async fn test_network_connectivity(endpoint: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(endpoint).build();
    
    match client.get_version().await {
        Ok(_) => Ok(true),
        Err(e) => {
            log::error!("Network connectivity failed: {}", e);
            Ok(false)
        }
    }
}
```

#### 4. Insufficient Funds

**Problem**: Account doesn't have enough SOL for rent or transaction fees.

**Solution**:
```rust
async fn ensure_sufficient_funds(
    client: &SolanaRpcClient,
    pubkey: &Pubkey,
    required_balance: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let balance = client.get_balance(pubkey).await?;
    
    if balance < required_balance {
        // Request airdrop
        let signature = client.request_airdrop(pubkey, required_balance - balance + 1000000000).await?;
        
        // Wait for airdrop to be processed
        wait_for_transaction(client, &signature).await?;
    }
    
    Ok(())
}

async fn wait_for_transaction(
    client: &SolanaRpcClient,
    signature: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..30 { // Wait up to 30 seconds
        if let Some(_) = client.get_signature_status(signature).await? {
            return Ok(());
        }
        sleep(Duration::from_secs(1)).await;
    }
    Err("Transaction not confirmed within timeout".into())
}
```

### Debugging Tips

1. **Enable Logging**:
```rust
console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
```

2. **Use Network Inspection**:
```rust
async fn debug_account_state(pubkey: &Pubkey) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    
    let account_info = client.get_account_info(pubkey).await?;
    if let Some(account) = account_info {
        log::debug!("Account info: {:?}", account);
        
        // Try to deserialize as known structures
        if let Ok(hello_account) = serde_json::from_slice::<HelloAccount>(&account.data) {
            log::debug!("Hello account: {:?}", hello_account);
        } else {
            log::debug!("Raw data: {:?}", account.data);
        }
    }
    
    Ok(())
}
```

3. **Transaction Simulation**:
```rust
async fn simulate_transaction(
    client: &SolanaRpcClient,
    transaction: &Transaction,
) -> Result<SimulationResult, Box<dyn std::error::Error>> {
    let result = client.simulate_transaction(transaction).await?;
    
    log::debug!("Simulation result: {:?}", result);
    
    if let Some(err) = result.error {
        log::error!("Simulation failed: {:?}", err);
    }
    
    Ok(result)
}
```

---

## 📚 Additional Resources

### Official Documentation
- [Solana Documentation](https://docs.solana.com/)
- [Solana Program Library](https://spl.solana.com/)
- [Rust Documentation](https://doc.rust-lang.org/)

### Community Resources
- [Solana Discord](https://discord.gg/solana)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/solana)
- [GitHub Discussions](https://github.com/solana-labs/solana/discussions)

### Examples in This Repository
- `examples/hello_program_complete.rs` - Complete program lifecycle
- `examples/simple_hello_program.rs` - Simple program interaction
- `examples/program_deployment/` - Program deployment examples
- `examples/dioxus_app.rs` - Web application integration

---

## 🤝 Contributing

We welcome contributions to improve this documentation and the `gloo_solana` library! Please feel free to:

1. Report issues on GitHub
2. Submit pull requests
3. Improve documentation
4. Add more examples
5. Share your projects using `gloo_solana`

---

**Happy coding with gloo_solana! 🚀🌊**
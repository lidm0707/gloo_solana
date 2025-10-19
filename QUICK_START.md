# 🚀 gloo_solana Quick Start Guide

Get up and running with Solana program development in minutes using gloo_solana!

## 📋 Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install surfpool (for local testing)
cargo install surfpool

# Start surfpool
surfpool start
```

## ⚡ 5-Minute Hello World

### 1. Add Dependencies

```toml
[dependencies]
gloo_solana = { version = "0.1.0", features = ["dioxus"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

### 2. Create Your First Program

```rust
use serde::{Deserialize, Serialize};
use gloo_solana::{surfpool_network, RpcClientBuilder, CommitmentLevel};

// Define your program's account data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloAccount {
    pub greeting: String,
    pub counter: u64,
    pub authority: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Hello World Program Demo");
    
    // Create RPC client
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // Test connectivity
    let version = client.get_version().await?;
    println!("✅ Connected to surfpool v{}", version.solana_core);

    // Create program account data
    let account_data = HelloAccount {
        greeting: "Hello from gloo_solana! 🚀".to_string(),
        counter: 0,
        authority: "demo_authority".to_string(),
    };

    // Serialize and display
    let serialized = serde_json::to_string_pretty(&account_data)?;
    println!("📝 Account data:\n{}", serialized);

    println!("✅ Hello World program ready!");
    Ok(())
}
```

### 3. Run Your Program

```bash
cargo run
```

Expected output:
```
🌊 Hello World Program Demo
✅ Connected to surfpool v2.3.8
📝 Account data:
{
  "greeting": "Hello from gloo_solana! 🚀",
  "counter": 0,
  "authority": "demo_authority"
}
✅ Hello World program ready!
```

## 🎯 Create and Call Programs

### Step 1: Define Program Structure

```rust
use gloo_solana::domain::types::Pubkey;

// Program instruction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GreetingInstruction {
    Initialize { greeting: String },
    UpdateGreeting { new_greeting: String },
    IncrementCounter,
    ResetCounter,
}

// Program account state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetingAccount {
    pub greeting: String,
    pub counter: u64,
    pub authority: Pubkey,
    pub created_at: u64,
}
```

### Step 2: Deploy Your Program

```rust
use gloo_solana::{
    application::services::programs::{ProgramService, ProgramDeployment, DeploymentConfig},
    domain::programs::Program,
};

async fn deploy_program() -> Result<Pubkey, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let mut program_service = ProgramService::new(client);

    // Create program
    let program = Program::new(
        generate_program_id("greeting_program"),
        "Greeting Program".to_string(),
        "1.0.0".to_string(),
        "A simple greeting program".to_string(),
        vec![1, 2, 3, 4], // Mock bytecode
        None,
    );

    // Configure deployment
    let config = DeploymentConfig {
        skip_preflight: true,
        max_compute_units: Some(200_000),
        priority_fee: Some(1000),
        commitment: "confirmed".to_string(),
    };

    // Deploy
    let deployment = ProgramDeployment::new(program, config);
    let program_id = program_service.deploy_program(deployment).await?;
    
    println!("🚀 Program deployed: {}", program_id);
    Ok(program_id)
}

fn generate_program_id(seed: &str) -> Pubkey {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(b"_program");
    let hash = hasher.finalize();
    
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);
    Pubkey::new(bytes)
}
```

### Step 3: Create Program Accounts

```rust
use gloo_solana::application::services::programs::AccountCreationService;

async fn create_greeting_account(
    program_id: &Pubkey,
    greeting: &str,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client.clone());
    let account_service = AccountCreationService::new(program_service);

    // Generate account keypair
    let account_keypair = generate_account_keypair("greeting_account");
    let account_pubkey = derive_pubkey(&account_keypair);

    // Create account with initial data
    let request = account_service.create_hello_account(
        *program_id,
        greeting.to_string(),
        account_pubkey,
    );

    let created_pubkey = program_service.create_account(request).await?;
    println!("📝 Account created: {}", created_pubkey);
    
    Ok(created_pubkey)
}
```

### Step 4: Call Your Program

```rust
use gloo_solana::application::services::programs::InstructionBuilder;

async fn call_greeting_program(
    program_id: &Pubkey,
    account_pubkey: &Pubkey,
    new_greeting: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);

    // Create instruction
    let instruction_builder = InstructionBuilder::new(*program_id);
    let instruction = instruction_builder.update_greeting(
        *account_pubkey,
        new_greeting.to_string(),
        generate_authority_pubkey(),
    );

    // Execute instruction
    program_service.execute_instruction(instruction).await?;
    println!("✅ Greeting updated to: {}", new_greeting);
    
    Ok(())
}
```

## 🌐 Web Integration with Dioxus

### 1. Set Up Dioxus App

```rust
use dioxus::prelude::*;
use gloo_solana::{
    dioxus_integration::{SolanaProvider, use_balance, use_account_info},
    surfpool_network,
};

#[component]
fn App() -> Element {
    rsx! {
        SolanaProvider { network: surfpool_network(),
+            GreetingProgram {}
        }
    }
}

#[component]
fn GreetingProgram() -> Element {
    let mut greeting = use_signal(|| "Hello World!".to_string());
    let mut counter = use_signal(|| 0u64);
    let mut loading = use_signal(|| false);

    let solana_context = use_context::<SolanaContext>().unwrap();
    
    // Reactive balance display
    let balance = use_balance(cx, &solana_context.client, solana_context.authority);

    let handle_update = move |_| {
        spawn(async move {
            loading.set(true);
            // Update greeting logic here
            loading.set(false);
        });
    };

    rsx! {
        div { class: "container",
            h1 { "🌊 gloo_solana Greeting Program" }
            
            div { class: "balance",
                "Balance: "
                match &*balance.read() {
                    Some(Ok(lamports)) => rsx! { "{lamports} lamports" },
                    Some(Err(e)) => rsx! { "Error: {e}" },
                    None => rsx! { "Loading..." }
                }
            }
            
            div { class: "controls",
                input {
                    value: "{greeting}",
                    oninput: move |e| greeting.set(e.value.clone())
                }
                button {
                    onclick: handle_update,
                    disabled: loading(),
                    "Update Greeting"
                }
                
                button {
                    onclick: move |_| counter += 1,
                    "Increment ({counter})"
                }
            }
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
```

### 2. Run Web App

```bash
# Install trunk for web building
cargo install trunk

# Build and serve
trunk serve
```

## 🛠️ Common Patterns

### Pattern 1: Account Management

```rust
pub struct AccountManager {
    client: SolanaRpcClient,
    program_id: Pubkey,
}

impl AccountManager {
    pub async fn create_account(&self, data: &[u8]) -> Result<Pubkey, ProgramError> {
        // 1. Calculate rent exemption
        let rent = self.client.get_minimum_balance_for_rent_exemption(data.len()).await?;
        
        // 2. Create account
        let account = self.create_account_with_rent(data, rent).await?;
        
        // 3. Initialize account data
        self.initialize_account(&account, data).await?;
        
        Ok(account)
    }
    
    pub async fn update_account(&self, pubkey: &Pubkey, new_data: &[u8]) -> Result<(), ProgramError> {
        let instruction = create_update_instruction(pubkey, new_data);
        self.execute_instruction(instruction).await?;
        Ok(())
    }
}
```

### Pattern 2: Batch Operations

```rust
pub async fn batch_update_greetings(
    updates: Vec<(Pubkey, String)>,
) -> Result<Vec<TransactionResult>, ProgramError> {
    let client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
    let program_service = ProgramService::new(client);
    
    let mut results = Vec::new();
    
    for (account_pubkey, new_greeting) in updates {
        let instruction = create_greeting_update_instruction(&account_pubkey, &new_greeting);
        
        match program_service.execute_instruction(instruction).await {
            Ok(signature) => results.push(TransactionResult::success(signature)),
            Err(e) => results.push(TransactionResult::error(e.to_string())),
        }
        
        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(results)
}
```

### Pattern 3: Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(Pubkey),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Instruction failed: {0}")]
    InstructionFailed(String),
}

pub async fn safe_program_call(
    program_id: &Pubkey,
    instruction: ProgramInstruction,
) -> Result<TransactionSignature, ProgramError> {
    // Pre-flight checks
    verify_program_exists(program_id).await?;
    verify_sufficient_balance().await?;
    
    // Execute instruction
    match execute_instruction(program_id, instruction).await {
        Ok(signature) => Ok(signature),
        Err(e) => Err(ProgramError::InstructionFailed(e.to_string())),
    }
}
```

## 📊 Testing Your Programs

### Unit Tests

```rust
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
    fn test_account_serialization() {
        let account = GreetingAccount {
            greeting: "Hello".to_string(),
            counter: 42,
            authority: Pubkey::new([1; 32]),
            created_at: 1234567890,
        };

        let serialized = serde_json::to_vec(&account).unwrap();
        let deserialized: GreetingAccount = serde_json::from_slice(&serialized).unwrap();
        
        assert_eq!(account.greeting, deserialized.greeting);
        assert_eq!(account.counter, deserialized.counter);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_program_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    // Start surfpool if not running
    if !is_surfpool_running().await? {
        start_surfpool().await?;
    }

    // Deploy program
    let program_id = deploy_test_program().await?;
    
    // Create account
    let account_pubkey = create_test_account(&program_id).await?;
    
    // Call program
    call_test_program(&program_id, &account_pubkey).await?;
    
    // Verify results
    let account_data = read_account_data(&account_pubkey).await?;
    assert_eq!(account_data.greeting, "Test Greeting");
    
    Ok(())
}
```

## 🔧 Development Workflow

### 1. Local Development

```bash
# Start surfpool
surfpool start

# Run tests
cargo test

# Run example
cargo run --example simple_hello_program

# Test web version
trunk serve
```

### 2. Deploy to Testnet

```rust
use gloo_solana::Network;

async fn deploy_to_testnet() -> Result<Pubkey, Box<dyn std::error::Error>> {
    let client = RpcClientBuilder::new(Network::Testnet.endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // Same deployment logic as local
    let program_id = deploy_program_with_client(client).await?;
    println!("🚀 Deployed to testnet: {}", program_id);
    
    Ok(program_id)
}
```

### 3. Production Deployment

```bash
# Build for production
cargo build --release

# Deploy to mainnet (with proper safety checks)
# WARNING: Only deploy after thorough testing!
```

## 🚨 Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| "Program not found" | Verify program ID and network endpoint |
| "Account not initialized" | Call initialize instruction first |
| "Insufficient funds" | Request airdrop or transfer SOL |
| Network timeout | Check surfpool is running, use proper commitment level |
| Serialization error | Ensure data structures match between client and program |

## 📚 Next Steps

1. **Explore Examples**: Check out `examples/` directory for more demos
2. **Read Full Guide**: See `PROGRAM_CREATION_GUIDE.md` for comprehensive documentation
3. **Join Community**: Get help on Discord or GitHub Discussions
4. **Build Something**: Create your own Solana dApp with gloo_solana!

## 🤝 Need Help?

- 📖 **Documentation**: `PROGRAM_CREATION_GUIDE.md`
- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/gloo_solana/issues)
- 💬 **Discord**: [Solana Discord](https://discord.gg/solana)
- 📧 **Email**: support@gloo-solana.dev

---

**🎉 Congratulations! You're ready to build Solana programs with gloo_solana!**

Happy coding! 🚀🌊
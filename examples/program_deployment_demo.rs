//! Program Deployment Demo Example
//!
//! This example demonstrates how to use gloo_solana's program management features
//! without requiring actual network connections or WASM compilation.
//! It simulates program deployment, account creation, and instruction execution.
//!
//! This demo shows:
//! 1. Creating and deploying programs
//! 2. Creating program accounts with custom data
//! 3. Executing program instructions
//! 4. Managing program lifecycle
//! 5. Account data serialization and storage

use gloo_solana::{
    application::services::programs::{AccountCreationService, InstructionBuilder, ProgramService},
    constants::SYSTEM_PROGRAM_ID,
    domain::{
        programs::{
            AccountMetadata, CreateAccountParams, CreateAccountRequest, DeploymentConfig, Program,
            ProgramAccount, ProgramDeployment, ProgramStatus,
        },
        types::Pubkey,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Hello World program data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloProgramData {
    pub greeting: String,
    pub counter: u64,
    pub last_updated: u64,
    pub owner: Pubkey,
}

/// Counter program data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterProgramData {
    pub value: u64,
    pub max_value: u64,
    pub increment_step: u64,
    pub owner: Pubkey,
    pub history: Vec<u64>,
}

/// Simulated network status for demo
#[derive(Debug, Clone)]
pub struct SimulatedNetworkStatus {
    pub is_connected: bool,
    pub block_height: u64,
    pub latest_blockhash: String,
    pub slot: u64,
    pub epoch: u64,
}

/// Simulated account information
#[derive(Debug, Clone)]
pub struct SimulatedAccount {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Simulated transaction result
#[derive(Debug, Clone)]
pub struct SimulatedTransactionResult {
    pub signature: String,
    pub success: bool,
    pub error: Option<String>,
    pub slot: u64,
    pub confirmations: u64,
}

/// Simulated RPC client for demo purposes
#[derive(Clone)]
pub struct SimulatedRpcClient {
    programs: HashMap<Pubkey, Program>,
    accounts: HashMap<Pubkey, SimulatedAccount>,
    block_height: u64,
    slot: u64,
}

impl SimulatedRpcClient {
    /// Create a new simulated RPC client
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            accounts: HashMap::new(),
            block_height: 1000,
            slot: 1000,
        }
    }

    /// Simulate getting account info
    pub fn get_account_info(&self, pubkey: &Pubkey) -> Option<&SimulatedAccount> {
        self.accounts.get(pubkey)
    }

    /// Simulate getting balance
    pub fn get_balance(&self, pubkey: &Pubkey) -> u64 {
        self.accounts
            .get(pubkey)
            .map(|account| account.lamports)
            .unwrap_or(0)
    }

    /// Simulate getting latest blockhash
    pub fn get_latest_blockhash(&self) -> SimulatedBlockhash {
        SimulatedBlockhash {
            blockhash: format!("blockhash_{}", self.block_height),
            last_valid_block_height: self.block_height + 150,
        }
    }

    /// Simulate getting block height
    pub fn get_block_height(&self) -> u64 {
        self.block_height
    }

    /// Simulate sending a transaction
    pub fn send_transaction(&self, _transaction: &str) -> SimulatedTransactionResult {
        SimulatedTransactionResult {
            signature: format!("signature_{}", self.slot),
            success: true,
            error: None,
            slot: self.slot,
            confirmations: 1,
        }
    }

    /// Add a program to the simulated client
    pub fn add_program(&mut self, program: Program) {
        self.programs.insert(program.program_id, program);
    }

    /// Add an account to the simulated client
    pub fn add_account(&mut self, account: SimulatedAccount) {
        self.accounts.insert(account.pubkey, account);
    }

    /// Increment block height
    pub fn increment_block_height(&mut self) {
        self.block_height += 1;
        self.slot += 1;
    }
}

/// Simulated latest blockhash
#[derive(Debug, Clone)]
pub struct SimulatedBlockhash {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

/// Simulated network service
pub struct SimulatedNetworkService {
    client: SimulatedRpcClient,
}

impl SimulatedNetworkService {
    /// Create a new simulated network service
    pub fn new(client: SimulatedRpcClient) -> Self {
        Self { client }
    }

    /// Get simulated network status
    pub fn get_network_status(&self) -> SimulatedNetworkStatus {
        SimulatedNetworkStatus {
            is_connected: true,
            block_height: self.client.get_block_height(),
            latest_blockhash: self.client.get_latest_blockhash().blockhash,
            slot: self.client.slot,
            epoch: self.client.block_height / 432000, // Approximate
        }
    }

    /// Simulate health check
    pub fn health_check(&self) -> bool {
        true
    }
}

/// Simulated account service
pub struct SimulatedAccountService {
    client: SimulatedRpcClient,
}

impl SimulatedAccountService {
    /// Create a new simulated account service
    pub fn new(client: SimulatedRpcClient) -> Self {
        Self { client }
    }

    /// Get account balance
    pub fn get_balance(&self, pubkey: &Pubkey) -> u64 {
        self.client.get_balance(pubkey)
    }

    /// Get account information
    pub fn get_account_info(&self, pubkey: &Pubkey) -> Option<&SimulatedAccount> {
        self.client.get_account_info(pubkey)
    }

    /// Get multiple account balances
    pub fn get_multiple_balances(&self, pubkeys: &[Pubkey]) -> Vec<Option<u64>> {
        pubkeys
            .iter()
            .map(|pubkey| Some(self.client.get_balance(pubkey)))
            .collect()
    }
}

/// Simulated transaction service
pub struct SimulatedTransactionService {
    client: SimulatedRpcClient,
}

impl SimulatedTransactionService {
    /// Create a new simulated transaction service
    pub fn new(client: SimulatedRpcClient) -> Self {
        Self { client }
    }

    /// Get latest blockhash
    pub fn get_latest_blockhash(&self) -> SimulatedBlockhash {
        self.client.get_latest_blockhash()
    }

    /// Send a transaction
    pub fn send_transaction(&self, transaction: &str) -> SimulatedTransactionResult {
        self.client.send_transaction(transaction)
    }

    /// Get block height
    pub fn get_block_height(&self) -> u64 {
        self.client.get_block_height()
    }
}

/// Enhanced program service for demo
#[derive(Clone)]
pub struct DemoProgramService {
    client: SimulatedRpcClient,
    deployed_programs: HashMap<Pubkey, Program>,
}

impl DemoProgramService {
    /// Create a new demo program service
    pub fn new(client: SimulatedRpcClient) -> Self {
        Self {
            client,
            deployed_programs: HashMap::new(),
        }
    }

    /// Deploy a new program (simulated)
    pub fn deploy_program(
        &mut self,
        deployment: ProgramDeployment,
    ) -> Result<Pubkey, Box<dyn Error>> {
        let program_id = deployment.program.program_id;

        println!("🚀 Deploying program: {}", deployment.program.name);
        println!("   Program ID: {}", program_id);
        println!("   Version: {}", deployment.program.version);
        println!("   Data size: {} bytes", deployment.program.data.len());

        // Simulate deployment process
        let mut program = deployment.program;
        program.mark_deployed();

        self.deployed_programs.insert(program_id, program.clone());
        self.client.add_program(program.clone());

        // Add some initial accounts to the client
        self.add_system_accounts(&program_id)?;

        println!("✅ Program deployed successfully!");
        Ok(program_id)
    }

    /// Create a program account
    pub fn create_account(
        &mut self,
        request: CreateAccountRequest,
    ) -> Result<Pubkey, Box<dyn Error>> {
        let account_pubkey = request.account.pubkey;

        println!("📝 Creating program account:");
        println!("   Account: {}", account_pubkey);
        println!("   Owner: {}", request.program_id);
        println!("   Size: {} bytes", request.params.space);
        println!("   Lamports: {}", request.params.lamports);

        // Create simulated account
        let simulated_account = SimulatedAccount {
            pubkey: account_pubkey,
            lamports: request.params.lamports + request.account.lamports,
            data: request.account.data.clone(),
            owner: request.program_id,
            executable: request.params.executable,
            rent_epoch: 0,
        };

        self.client.add_account(simulated_account);

        // Add to deployed program's accounts if we have it
        if let Some(program) = self.deployed_programs.get_mut(&request.program_id) {
            program.add_account(request.account.clone());
        }

        println!("✅ Account created successfully!");
        Ok(account_pubkey)
    }

    /// Execute a program instruction
    pub fn execute_instruction(&mut self, instruction: &str) -> Result<(), Box<dyn Error>> {
        println!("⚡ Executing program instruction:");
        println!("   Instruction: {}", instruction);

        // Simulate instruction execution
        self.client.increment_block_height();

        println!("✅ Instruction executed successfully!");
        Ok(())
    }

    /// Get program information
    pub fn get_program(&self, program_id: &Pubkey) -> Option<&Program> {
        self.deployed_programs.get(program_id)
    }

    /// Get account information from a program
    pub fn get_program_account(
        &self,
        program_id: &Pubkey,
        account_pubkey: &Pubkey,
    ) -> Option<&ProgramAccount> {
        self.deployed_programs
            .get(program_id)
            .and_then(|program| program.get_account(account_pubkey))
    }

    /// List all deployed programs
    pub fn list_programs(&self) -> Vec<&Program> {
        self.deployed_programs.values().collect()
    }

    /// Get program statistics
    pub fn get_program_stats(&self, program_id: &Pubkey) -> Option<ProgramStats> {
        self.deployed_programs
            .get(program_id)
            .map(|program| ProgramStats {
                program_id: program.program_id,
                name: program.name.clone(),
                version: program.version.clone(),
                account_count: program.accounts.len(),
                total_size: program.total_accounts_size(),
                total_lamports: program.total_lamports(),
                status: program.status.clone(),
                deployed_at: program.deployed_at,
            })
    }

    /// Add system accounts for a program
    fn add_system_accounts(&mut self, program_id: &Pubkey) -> Result<(), Box<dyn Error>> {
        // Add system program account
        let system_account = SimulatedAccount {
            pubkey: SYSTEM_PROGRAM_ID,
            lamports: 1_000_000_000,
            data: vec![0; 8], // Minimal system account data
            owner: *program_id,
            executable: true,
            rent_epoch: 0,
        };
        self.client.add_account(system_account);

        // Add clock sysvar account
        let clock_pubkey = generate_sysvar_pubkey("clock");
        let clock_account = SimulatedAccount {
            pubkey: clock_pubkey,
            lamports: 1_000_000,
            data: vec![0; 40], // Clock sysvar data size
            owner: *program_id,
            executable: false,
            rent_epoch: 0,
        };
        self.client.add_account(clock_account);

        Ok(())
    }
}

/// Statistics for a deployed program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramStats {
    pub program_id: Pubkey,
    pub name: String,
    pub version: String,
    pub account_count: usize,
    pub total_size: usize,
    pub total_lamports: u64,
    pub status: ProgramStatus,
    pub deployed_at: u64,
}

/// Enhanced account creation service for demo
pub struct DemoAccountCreationService {
    program_service: DemoProgramService,
}

impl DemoAccountCreationService {
    /// Create a new demo account creation service
    pub fn new(program_service: DemoProgramService) -> Self {
        Self { program_service }
    }

    /// Create a hello world program account
    pub fn create_hello_account(
        &self,
        program_id: Pubkey,
        user_name: String,
        message: String,
        owner: Pubkey,
    ) -> CreateAccountRequest {
        let account_data = HelloProgramData {
            greeting: message.clone(),
            counter: 0,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            owner,
        };

        let serialized_data = serde_json::to_vec(&account_data).unwrap();
        let account_pubkey = self.generate_pda(&program_id, &user_name);

        let account = ProgramAccount::new(
            account_pubkey,
            program_id,
            serialized_data.clone(),
            1_000_000, // 0.001 SOL
            false,
            AccountMetadata {
                name: format!("hello_account_{}", user_name),
                account_type: "hello_world".to_string(),
                description: "Hello world program account".to_string(),
                mutable: true,
                seeds: vec![user_name.as_bytes().to_vec()],
            },
        );

        let params = CreateAccountParams {
            space: serialized_data.len() as u64,
            lamports: 1_000_000,
            executable: false,
            seeds: Some(vec![user_name.as_bytes().to_vec()]),
        };

        CreateAccountRequest::new(account, program_id, owner, params)
    }

    /// Create a counter program account
    pub fn create_counter_account(
        &self,
        program_id: Pubkey,
        counter_name: String,
        initial_value: u64,
        owner: Pubkey,
    ) -> CreateAccountRequest {
        let account_data = CounterProgramData {
            value: initial_value,
            max_value: u64::MAX / 2,
            increment_step: 1,
            owner,
            history: vec![initial_value],
        };

        let serialized_data = serde_json::to_vec(&account_data).unwrap();
        let account_pubkey = self.generate_pda(&program_id, &counter_name);

        let account = ProgramAccount::new(
            account_pubkey,
            program_id,
            serialized_data.clone(),
            2_000_000, // 0.002 SOL
            false,
            AccountMetadata {
                name: format!("counter_{}", counter_name),
                account_type: "counter".to_string(),
                description: "Counter program account".to_string(),
                mutable: true,
                seeds: vec![counter_name.as_bytes().to_vec()],
            },
        );

        let params = CreateAccountParams {
            space: serialized_data.len() as u64,
            lamports: 2_000_000,
            executable: false,
            seeds: Some(vec![counter_name.as_bytes().to_vec()]),
        };

        CreateAccountRequest::new(account, program_id, owner, params)
    }

    /// Create a storage program account
    pub fn create_storage_account(
        &self,
        program_id: Pubkey,
        storage_name: String,
        data: Vec<u8>,
        owner: Pubkey,
    ) -> CreateAccountRequest {
        let account_pubkey = self.generate_pda(&program_id, &storage_name);

        let account = ProgramAccount::new(
            account_pubkey,
            program_id,
            data.clone(),
            (data.len() as u64) * 1_000_000, // 1 SOL per KB
            false,
            AccountMetadata {
                name: format!("storage_{}", storage_name),
                account_type: "storage".to_string(),
                description: "Storage account for program data".to_string(),
                mutable: true,
                seeds: vec![storage_name.as_bytes().to_vec()],
            },
        );

        let params = CreateAccountParams {
            space: data.len() as u64,
            lamports: (data.len() as u64) * 1_000_000,
            executable: false,
            seeds: Some(vec![storage_name.as_bytes().to_vec()]),
        };

        CreateAccountRequest::new(account, program_id, owner, params)
    }

    /// Generate a Program Derived Address (PDA)
    fn generate_pda(&self, program_id: &Pubkey, seed: &str) -> Pubkey {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        hasher.update(program_id.as_bytes());
        let hash = hasher.finalize();

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash);

        Pubkey::new(bytes)
    }
}

/// Instruction builder for demo
pub struct DemoInstructionBuilder {
    program_id: Pubkey,
}

impl DemoInstructionBuilder {
    /// Create a new instruction builder
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }

    /// Build a hello world instruction
    pub fn hello_world(&self, user_account: Pubkey, payer: Pubkey) -> String {
        format!(
            "HelloWorld instruction: user={}, payer={}",
            user_account, payer
        )
    }

    /// Build a counter increment instruction
    pub fn increment_counter(&self, counter_account: Pubkey, payer: Pubkey) -> String {
        format!(
            "IncrementCounter instruction: counter={}, payer={}",
            counter_account, payer
        )
    }

    /// Build a counter set instruction
    pub fn set_counter(&self, counter_account: Pubkey, value: u64, payer: Pubkey) -> String {
        format!(
            "SetCounter instruction: counter={}, value={}, payer={}",
            counter_account, value, payer
        )
    }

    /// Build a storage write instruction
    pub fn write_storage(&self, storage_account: Pubkey, data: &[u8], payer: Pubkey) -> String {
        format!(
            "WriteStorage instruction: storage={}, size={}, payer={}",
            storage_account,
            data.len(),
            payer
        )
    }
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

/// Generate a sysvar pubkey
fn generate_sysvar_pubkey(sysvar: &str) -> Pubkey {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(b"Sysvar");
    hasher.update(sysvar.as_bytes());
    let hash = hasher.finalize();

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&hash);

    Pubkey::new(bytes)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Program Deployment Demo Example");
    println!("=================================");
    println!();
    println!("This example demonstrates program deployment and account management");
    println!("using gloo_solana's program features without network dependencies.");
    println!();

    // Create simulated RPC client
    let client = SimulatedRpcClient::new();

    println!("✅ Created simulated RPC client");

    // Create services
    let mut program_service = DemoProgramService::new(client.clone());
    let account_service = SimulatedAccountService::new(client.clone());
    let transaction_service = SimulatedTransactionService::new(client.clone());
    let network_service = SimulatedNetworkService::new(client.clone());

    println!("\n📊 Network Status:");
    let network_status = network_service.get_network_status();
    println!("   Connected: {}", network_status.is_connected);
    println!("   Block Height: {}", network_status.block_height);
    println!("   Latest Blockhash: {}", network_status.latest_blockhash);
    println!("   Slot: {}", network_status.slot);
    println!("   Epoch: {}", network_status.epoch);

    // Deploy Hello World program
    println!("\n🌟 Deploying Hello World Program:");
    let hello_program_id = deploy_hello_world_program(&mut program_service)?;

    // Deploy Counter program
    println!("\n🔢 Deploying Counter Program:");
    let counter_program_id = deploy_counter_program(&mut program_service)?;

    // Create program accounts
    println!("\n📝 Creating Program Accounts:");
    let hello_accounts = create_hello_accounts(&mut program_service, &hello_program_id)?;
    let counter_accounts = create_counter_accounts(&mut program_service, &counter_program_id)?;
    let storage_accounts = create_storage_accounts(&mut program_service, &hello_program_id)?;

    // Execute program instructions
    println!("\n⚡ Executing Program Instructions:");
    execute_hello_instructions(&mut program_service, &hello_program_id, &hello_accounts)?;
    execute_counter_instructions(&mut program_service, &counter_program_id, &counter_accounts)?;
    execute_storage_instructions(&mut program_service, &hello_program_id, &storage_accounts)?;

    // Show program statistics
    println!("\n📈 Program Statistics:");
    show_program_stats(&program_service, &hello_program_id)?;
    show_program_stats(&program_service, &counter_program_id)?;

    // Demonstrate account management
    println!("\n🔧 Account Management Demo:");
    demonstrate_account_management(&account_service, &transaction_service)?;

    // Show total statistics
    println!("\n📊 Total System Statistics:");
    show_total_statistics(&program_service, &account_service);

    println!("\n🎉 Program Deployment Demo Completed Successfully!");
    println!();
    println!("💡 Key Features Demonstrated:");
    println!("   ✅ Program deployment simulation");
    println!("   ✅ Account creation with custom data");
    println!("   ✅ Program instruction execution");
    println!("   ✅ Program Derived Address (PDA) generation");
    println!("   ✅ Account data serialization/deserialization");
    println!("   ✅ Program lifecycle management");
    println!("   ✅ Statistics and monitoring");
    println!();
    println!("🔗 Real-World Usage:");
    println!("   1. Use these patterns with actual Solana networks");
    println!("   2. Combine with full solana-sdk for deployment");
    println!("   3. Use gloo_solana for web-based account management");
    println!("   4. Build scalable dApps with these patterns");

    Ok(())
}

/// Deploy a Hello World program
fn deploy_hello_world_program(
    program_service: &mut DemoProgramService,
) -> Result<Pubkey, Box<dyn Error>> {
    let program_id = generate_program_id("hello_world");

    let program = Program::new(
        program_id,
        "Hello World".to_string(),
        "1.0.0".to_string(),
        "A simple hello world program that greets users and maintains counters".to_string(),
        vec![1, 2, 3, 4, 5], // Mock program data
        Some(SYSTEM_PROGRAM_ID),
    );

    let deployment_config = DeploymentConfig {
        skip_preflight: true,
        max_compute_units: Some(200_000),
        priority_fee: Some(1000),
        commitment: "confirmed".to_string(),
    };

    let deployment = ProgramDeployment::new(program, deployment_config);
    program_service.deploy_program(deployment)
}

/// Deploy a Counter program
fn deploy_counter_program(
    program_service: &mut DemoProgramService,
) -> Result<Pubkey, Box<dyn Error>> {
    let program_id = generate_program_id("counter");

    let program = Program::new(
        program_id,
        "Counter".to_string(),
        "1.0.0".to_string(),
        "A counter program that maintains state and supports increment operations".to_string(),
        vec![5, 6, 7, 8, 9, 10], // Mock program data
        Some(SYSTEM_PROGRAM_ID),
    );

    let deployment_config = DeploymentConfig {
        skip_preflight: false,
        max_compute_units: Some(150_000),
        priority_fee: Some(500),
        commitment: "confirmed".to_string(),
    };

    let deployment = ProgramDeployment::new(program, deployment_config);
    program_service.deploy_program(deployment)
}

/// Create accounts for Hello World program
fn create_hello_accounts(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let account_service = DemoAccountCreationService::new(program_service.clone());
    let mut created_accounts = Vec::new();

    let users = vec![
        ("alice", "Hello from Alice! 👋"),
        ("bob", "Bob says hi! 👋"),
        ("charlie", "Charlie's greeting! 🌟"),
        ("diana", "Diana waves hello! 👋"),
        ("eve", "Eve sends greetings! 🎉"),
    ];

    for (username, message) in users {
        let request = account_service.create_hello_account(
            *program_id,
            username.to_string(),
            message.to_string(),
            generate_payer_pubkey(username),
        );

        let account_pubkey = program_service.create_account(request)?;
        created_accounts.push(account_pubkey);

        println!("   Created account for {}: {}", username, account_pubkey);
    }

    Ok(created_accounts)
}

/// Create accounts for Counter program
fn create_counter_accounts(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let account_service = DemoAccountCreationService::new(program_service.clone());
    let mut created_accounts = Vec::new();

    let counters = vec![
        ("global_counter", 0),
        ("user_counter", 100),
        ("session_counter", 1000),
        ("event_counter", 42),
        ("click_counter", 777),
    ];

    for (counter_name, initial_value) in counters {
        let request = account_service.create_counter_account(
            *program_id,
            counter_name.to_string(),
            initial_value,
            generate_payer_pubkey(counter_name),
        );

        let account_pubkey = program_service.create_account(request)?;
        created_accounts.push(account_pubkey);

        println!(
            "   Created counter {}: {} (initial: {})",
            counter_name, account_pubkey, initial_value
        );
    }

    Ok(created_accounts)
}

/// Create storage accounts
fn create_storage_accounts(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
) -> Result<Vec<Pubkey>, Box<dyn Error>> {
    let account_service = DemoAccountCreationService::new(program_service.clone());
    let mut created_accounts = Vec::new();

    let storage_data = vec![
        (
            "user_profiles",
            b"{\"alice\": {\"name\": \"Alice\", \"age\": 30}}".to_vec(),
        ),
        (
            "config",
            b"{\"theme\": \"dark\", \"language\": \"en\"}".to_vec(),
        ),
        ("cache", b"cached_data_12345".to_vec()),
        (
            "logs",
            b"[{\"timestamp\": 1234567890, \"event\": \"login\"}]".to_vec(),
        ),
    ];

    for (storage_name, data) in storage_data {
        let request = account_service.create_storage_account(
            *program_id,
            storage_name.to_string(),
            data.to_vec(),
            generate_payer_pubkey(storage_name),
        );

        let account_pubkey = program_service.create_account(request)?;
        created_accounts.push(account_pubkey);

        println!(
            "   Created storage {}: {} ({} bytes)",
            storage_name,
            account_pubkey,
            data.len()
        );
    }

    Ok(created_accounts)
}

/// Execute Hello World program instructions
fn execute_hello_instructions(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
    accounts: &[Pubkey],
) -> Result<(), Box<dyn Error>> {
    let instruction_builder = DemoInstructionBuilder::new(*program_id);

    for (i, &account_pubkey) in accounts.iter().enumerate() {
        let instruction =
            instruction_builder.hello_world(account_pubkey, generate_payer_pubkey("payer"));

        println!(
            "   Executing Hello instruction {} for account {}",
            i + 1,
            account_pubkey
        );
        program_service.execute_instruction(&instruction)?;

        // Simulate account data update
        if let Some(account) = program_service.get_program_account(program_id, &account_pubkey) {
            println!("     Account data: {} bytes", account.size);
        }
    }

    Ok(())
}

/// Execute Counter program instructions
fn execute_counter_instructions(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
    accounts: &[Pubkey],
) -> Result<(), Box<dyn Error>> {
    let instruction_builder = DemoInstructionBuilder::new(*program_id);

    // Increment each counter
    for (i, &account_pubkey) in accounts.iter().enumerate() {
        let instruction =
            instruction_builder.increment_counter(account_pubkey, generate_payer_pubkey("payer"));

        println!(
            "   Incrementing counter {} for account {}",
            i + 1,
            account_pubkey
        );
        program_service.execute_instruction(&instruction)?;

        // Simulate counter value update
        println!("     Counter value updated");
    }

    // Set specific values
    if !accounts.is_empty() {
        let set_instruction =
            instruction_builder.set_counter(accounts[0], 42, generate_payer_pubkey("payer"));

        println!("   Setting counter {} to value 42", accounts[0]);
        program_service.execute_instruction(&set_instruction)?;
        println!("     Counter value set to 42");
    }

    Ok(())
}

/// Execute storage instructions
fn execute_storage_instructions(
    program_service: &mut DemoProgramService,
    program_id: &Pubkey,
    accounts: &[Pubkey],
) -> Result<(), Box<dyn Error>> {
    let instruction_builder = DemoInstructionBuilder::new(*program_id);

    for (i, &account_pubkey) in accounts.iter().enumerate() {
        let new_data = format!("updated_data_{}", i);
        let instruction = instruction_builder.write_storage(
            account_pubkey,
            new_data.as_bytes(),
            generate_payer_pubkey("payer"),
        );

        println!(
            "   Writing storage {} for account {}",
            i + 1,
            account_pubkey
        );
        program_service.execute_instruction(&instruction)?;

        // Simulate data update
        println!("     Storage data updated");
    }

    Ok(())
}

/// Show program statistics
fn show_program_stats(
    program_service: &DemoProgramService,
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

/// Demonstrate account management
fn demonstrate_account_management(
    account_service: &SimulatedAccountService,
    transaction_service: &SimulatedTransactionService,
) -> Result<(), Box<dyn Error>> {
    println!("   Checking system program account...");

    let system_account = account_service.get_account_info(&SYSTEM_PROGRAM_ID);

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
    let latest_blockhash = transaction_service.get_latest_blockhash();
    println!("   ✅ Latest blockhash: {}", latest_blockhash.blockhash);

    println!("   Getting current block height...");
    let block_height = transaction_service.get_block_height();
    println!("   ✅ Current block height: {}", block_height);

    // Simulate transaction
    println!("   Simulating transaction...");
    let transaction_result = transaction_service.send_transaction("demo_transaction");
    println!(
        "   ✅ Transaction: {} (success: {})",
        transaction_result.signature, transaction_result.success
    );

    Ok(())
}

/// Show total system statistics
fn show_total_statistics(
    program_service: &DemoProgramService,
    _account_service: &SimulatedAccountService,
) -> Result<(), Box<dyn Error>> {
    let programs = program_service.list_programs();
    let total_accounts: usize = programs.iter().map(|p| p.accounts.len()).sum();
    let total_size: usize = programs.iter().map(|p| p.total_accounts_size()).sum();
    let total_lamports: u64 = programs.iter().map(|p| p.total_lamports()).sum();

    println!("   Total Programs: {}", programs.len());
    println!("   Total Accounts: {}", total_accounts);
    println!("   Total Storage: {} bytes", total_size);
    println!("   Total Lamports: {}", total_lamports);

    // Show account breakdown
    let mut lamports_by_owner = std::collections::HashMap::new();
    for program in programs {
        for account in program.accounts.values() {
            *lamports_by_owner.entry(account.owner).or_insert(0) += account.lamports;
        }
    }

    println!("   Lamports by Owner:");
    for (owner, lamports) in lamports_by_owner.iter().take(5) {
        println!("     {}: {} lamports", owner, lamports);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::constants::SYSTEM_PROGRAM_ID;

    #[test]
    fn test_simulated_client_creation() {
        let client = SimulatedRpcClient::new();
        assert_eq!(client.get_block_height(), 1000);
        assert_eq!(client.get_slot(), 1000);
    }

    #[test]
    fn test_program_id_generation() {
        let id1 = generate_program_id("test");
        let id2 = generate_program_id("test");
        let id3 = generate_program_id("different");

        assert_eq!(id1, id2); // Deterministic
        assert_ne!(id1, id3); // Different seeds
    }

    #[test]
    fn test_hello_account_creation() {
        let client = SimulatedRpcClient::new();
        let program_service = DemoProgramService::new(client);
        let account_service = DemoAccountCreationService::new(program_service);

        let program_id = generate_program_id("test_program");
        let request = account_service.create_hello_account(
            program_id,
            "test_user".to_string(),
            "Hello World!".to_string(),
            generate_payer_pubkey("test_user"),
        );

        assert_eq!(request.account.owner, program_id);
        assert_eq!(request.params.lamports, 1_000_000);
        assert!(request.params.space > 0);
    }

    #[test]
    fn test_instruction_builder() {
        let program_id = generate_program_id("test_program");
        let user_account = generate_payer_pubkey("user");
        let payer = generate_payer_pubkey("payer");

        let builder = DemoInstructionBuilder::new(program_id);
        let instruction = builder.hello_world(user_account, payer);

        assert!(instruction.contains("HelloWorld"));
        assert!(instruction.contains(&user_account.to_string()));
        assert!(instruction.contains(&payer.to_string()));
    }

    #[test]
    fn test_pda_generation() {
        let program_id = generate_program_id("test_program");
        let pda1 = generate_pda(&program_id, "seed1");
        let pda2 = generate_pda(&program_id, "seed2");

        assert_ne!(pda1, pda2);
        assert_eq!(pda1.to_base58().len(), 44); // Standard Solana pubkey length
    }

    #[test]
    fn test_serialization_roundtrip() {
        let data = HelloProgramData {
            greeting: "Hello".to_string(),
            counter: 42,
            last_updated: 1234567890,
            owner: SYSTEM_PROGRAM_ID,
        };

        let serialized = serde_json::to_vec(&data).unwrap();
        let deserialized: HelloProgramData = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(data.greeting, deserialized.greeting);
        assert_eq!(data.counter, deserialized.counter);
        assert_eq!(data.owner, deserialized.owner);
    }
}

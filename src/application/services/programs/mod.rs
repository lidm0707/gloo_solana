//! Program services module for managing Solana programs and accounts
//!
//! This module provides high-level services for program deployment,
//! account management, and program interaction using the gloo_solana library.

use crate::domain::programs::{
    AccountMetadata, CreateAccountParams, CreateAccountRequest, InstructionAccount, Program,
    ProgramAccount, ProgramDeployment, ProgramInstruction, ProgramStatus,
};
use crate::domain::types::Pubkey;
use crate::infrastructure::rpc::SolanaRpcClient;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::error::Error;

/// High-level service for managing Solana programs
#[derive(Clone)]
pub struct ProgramService {
    rpc_client: SolanaRpcClient,
    deployed_programs: HashMap<Pubkey, Program>,
}

impl ProgramService {
    /// Create a new program service
    pub fn new(rpc_client: SolanaRpcClient) -> Self {
        Self {
            rpc_client,
            deployed_programs: HashMap::new(),
        }
    }

    /// Deploy a new program (simulated deployment)
    pub async fn deploy_program(
        &mut self,
        deployment: ProgramDeployment,
    ) -> Result<Pubkey, Box<dyn Error>> {
        let program_id = deployment.program.program_id;

        // Simulate deployment process
        println!("🚀 Deploying program: {}", deployment.program.name);
        println!("   Program ID: {}", program_id);
        println!("   Version: {}", deployment.program.version);
        println!("   Data size: {} bytes", deployment.program.data.len());

        // In a real implementation, this would:
        // 1. Create deployment transaction
        // 2. Get latest blockhash
        // 3. Sign transaction
        // 4. Send transaction
        // 5. Wait for confirmation

        // For now, we simulate successful deployment
        let mut program = deployment.program;
        program.mark_deployed();

        self.deployed_programs.insert(program_id, program.clone());

        println!("✅ Program deployed successfully!");
        Ok(program_id)
    }

    /// Create a program account
    pub async fn create_account(
        &mut self,
        request: CreateAccountRequest,
    ) -> Result<Pubkey, Box<dyn Error>> {
        let account_pubkey = request.account.pubkey;

        println!("📝 Creating program account:");
        println!("   Account: {}", account_pubkey);
        println!("   Owner: {}", request.program_id);
        println!("   Size: {} bytes", request.params.space);
        println!("   Lamports: {}", request.params.lamports);

        // Simulate account creation
        // In a real implementation, this would create and send a transaction

        // Add to deployed program's accounts if we have it
        if let Some(program) = self.deployed_programs.get_mut(&request.program_id) {
            program.add_account(request.account.clone());
        }

        println!("✅ Account created successfully!");
        Ok(account_pubkey)
    }

    /// Execute a program instruction
    pub async fn execute_instruction(
        &self,
        instruction: ProgramInstruction,
    ) -> Result<(), Box<dyn Error>> {
        println!("⚡ Executing program instruction:");
        println!("   Program: {}", instruction.program_id);
        println!("   Instruction ID: {}", instruction.instruction_id);
        println!("   Data: {} bytes", instruction.data.len());
        println!("   Accounts: {}", instruction.accounts.len());

        for (i, account) in instruction.accounts.iter().enumerate() {
            println!(
                "     {}: {} ({:?}, signer: {}, writable: {})",
                i, account.pubkey, account.role, account.is_signer, account.is_writable
            );
        }

        // Simulate instruction execution
        // In a real implementation, this would create and send a transaction

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

/// Service for creating program accounts
pub struct AccountCreationService {
    program_service: ProgramService,
}

impl AccountCreationService {
    /// Create a new account creation service
    pub fn new(program_service: ProgramService) -> Self {
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
        let account_data = HelloAccountData {
            name: user_name.clone(),
            message,
            greeting_count: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
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
        let account_data = CounterAccountData {
            name: counter_name.clone(),
            value: initial_value,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
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

    /// Generate a Program Derived Address (PDA)
    fn generate_pda(&self, program_id: &Pubkey, seed: &str) -> Pubkey {
        // Simple PDA generation for demonstration
        // In a real implementation, this would use Solana's find_program_address
        let mut seeds = seed.as_bytes().to_vec();
        seeds.extend_from_slice(program_id.as_bytes());

        // Create a deterministic hash
        let hash = sha2::Sha256::digest(&seeds);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash);

        Pubkey::new(bytes)
    }
}

/// Hello world account data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloAccountData {
    pub name: String,
    pub message: String,
    pub greeting_count: u64,
    pub created_at: u64,
}

/// Counter account data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterAccountData {
    pub name: String,
    pub value: u64,
    pub last_updated: u64,
}

/// Service for building program instructions
pub struct InstructionBuilder {
    program_id: Pubkey,
}

impl InstructionBuilder {
    /// Create a new instruction builder
    pub fn new(program_id: Pubkey) -> Self {
        Self { program_id }
    }

    /// Build a hello world instruction
    pub fn hello_world(&self, user_account: Pubkey, payer: Pubkey) -> ProgramInstruction {
        let instruction_data = HelloInstruction::Greet;
        let serialized = serde_json::to_vec(&instruction_data).unwrap();

        let accounts = vec![
            InstructionAccount::program(self.program_id),
            InstructionAccount::writable(user_account),
            InstructionAccount::signer(payer, true),
        ];

        ProgramInstruction::new(self.program_id, accounts, serialized, 0)
    }

    /// Build a counter increment instruction
    pub fn increment_counter(&self, counter_account: Pubkey, payer: Pubkey) -> ProgramInstruction {
        let instruction_data = CounterInstruction::Increment;
        let serialized = serde_json::to_vec(&instruction_data).unwrap();

        let accounts = vec![
            InstructionAccount::program(self.program_id),
            InstructionAccount::writable(counter_account),
            InstructionAccount::signer(payer, true),
        ];

        ProgramInstruction::new(self.program_id, accounts, serialized, 1)
    }

    /// Build a counter set instruction
    pub fn set_counter(
        &self,
        counter_account: Pubkey,
        value: u64,
        payer: Pubkey,
    ) -> ProgramInstruction {
        let instruction_data = CounterInstruction::Set { value };
        let serialized = serde_json::to_vec(&instruction_data).unwrap();

        let accounts = vec![
            InstructionAccount::program(self.program_id),
            InstructionAccount::writable(counter_account),
            InstructionAccount::signer(payer, true),
        ];

        ProgramInstruction::new(self.program_id, accounts, serialized, 2)
    }
}

/// Hello world program instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HelloInstruction {
    Greet,
    UpdateMessage { new_message: String },
}

/// Counter program instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CounterInstruction {
    Increment,
    Set { value: u64 },
    Reset,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::constants::SYSTEM_PROGRAM_ID;

    #[test]
    fn test_program_service_creation() {
        let rpc_client =
            crate::infrastructure::rpc::RpcClientBuilder::new("http://localhost:8899").build();
        let service = ProgramService::new(rpc_client);

        assert_eq!(service.deployed_programs.len(), 0);
    }

    #[test]
    fn test_hello_account_creation() {
        let rpc_client =
            crate::infrastructure::rpc::RpcClientBuilder::new("http://localhost:8899").build();
        let program_service = ProgramService::new(rpc_client);
        let account_service = AccountCreationService::new(program_service);

        let program_id = Pubkey::new([1; 32]);
        let owner = Pubkey::new([2; 32]);

        let request = account_service.create_hello_account(
            program_id,
            "test_user".to_string(),
            "Hello World!".to_string(),
            owner,
        );

        assert_eq!(request.account.owner, program_id);
        assert_eq!(request.payer, owner);
        assert!(request.params.space > 0);
    }

    #[test]
    fn test_instruction_builder() {
        let program_id = Pubkey::new([1; 32]);
        let user_account = Pubkey::new([2; 32]);
        let payer = Pubkey::new([3; 32]);

        let builder = InstructionBuilder::new(program_id);
        let instruction = builder.hello_world(user_account, payer);

        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 3);
        assert_eq!(instruction.instruction_id, 0);
    }

    #[test]
    fn test_counter_instruction_builder() {
        let program_id = Pubkey::new([1; 32]);
        let counter_account = Pubkey::new([2; 32]);
        let payer = Pubkey::new([3; 32]);

        let builder = InstructionBuilder::new(program_id);
        let instruction = builder.increment_counter(counter_account, payer);

        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 3);
        assert_eq!(instruction.instruction_id, 1);
    }
}

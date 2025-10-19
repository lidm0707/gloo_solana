//! Program domain module for Solana program account management
//!
//! This module provides types and functionality for working with Solana programs
//! and their associated accounts, including program deployment simulation,
//! account creation, and program interaction patterns.

use crate::domain::types::{Hash, Pubkey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Solana program with its metadata and accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    /// The program's public key
    pub program_id: Pubkey,
    /// Program name for identification
    pub name: String,
    /// Program version
    pub version: String,
    /// Program description
    pub description: String,
    /// Program authority (upgrade authority)
    pub authority: Option<Pubkey>,
    /// Program data (binary)
    pub data: Vec<u8>,
    /// Accounts owned by this program
    pub accounts: HashMap<Pubkey, ProgramAccount>,
    /// Program deployment status
    pub status: ProgramStatus,
    /// Deployment timestamp
    pub deployed_at: u64,
}

/// Status of a program deployment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgramStatus {
    /// Program is being deployed
    Deploying,
    /// Program is deployed and active
    Deployed,
    /// Program failed to deploy
    Failed,
    /// Program is being upgraded
    Upgrading,
    /// Program is closed/invalid
    Closed,
}

/// An account owned by a program
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramAccount {
    /// The account's public key
    pub pubkey: Pubkey,
    /// Account owner (program ID)
    pub owner: Pubkey,
    /// Account data
    pub data: Vec<u8>,
    /// Account balance in lamports
    pub lamports: u64,
    /// Whether the account is executable
    pub executable: bool,
    /// Account size in bytes
    pub size: usize,
    /// Account creation timestamp
    pub created_at: u64,
    /// Account metadata
    pub metadata: AccountMetadata,
}

/// Metadata for program accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountMetadata {
    /// Account name/identifier
    pub name: String,
    /// Account type or purpose
    pub account_type: String,
    /// Account description
    pub description: String,
    /// Whether this account is mutable
    pub mutable: bool,
    /// Account seeds for PDA generation
    pub seeds: Vec<Vec<u8>>,
}

/// Program instruction for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDeployment {
    /// Program to deploy
    pub program: Program,
    /// Deployment configuration
    pub config: DeploymentConfig,
    /// Required signatures
    pub required_signatures: Vec<Pubkey>,
    /// Deployment fee in lamports
    pub fee: u64,
}

/// Configuration for program deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Whether to skip pre-flight checks
    pub skip_preflight: bool,
    /// Maximum compute units
    pub max_compute_units: Option<u32>,
    /// Priority fee in lamports
    pub priority_fee: Option<u64>,
    /// Commitment level
    pub commitment: String,
}

/// Program account creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    /// Account to create
    pub account: ProgramAccount,
    /// Program that will own the account
    pub program_id: Pubkey,
    /// Payer for account creation
    pub payer: Pubkey,
    /// Account creation parameters
    pub params: CreateAccountParams,
}

/// Parameters for creating a program account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountParams {
    /// Space to allocate for the account
    pub space: u64,
    /// Lamports to rent for the account
    pub lamports: u64,
    /// Whether the account should be executable
    pub executable: bool,
    /// Account seeds for PDA (if applicable)
    pub seeds: Option<Vec<Vec<u8>>>,
}

/// Program instruction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInstruction {
    /// Program ID to execute
    pub program_id: Pubkey,
    /// Accounts involved in the instruction
    pub accounts: Vec<InstructionAccount>,
    /// Instruction data
    pub data: Vec<u8>,
    /// Instruction identifier
    pub instruction_id: u8,
}

/// Account involved in a program instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionAccount {
    /// Account public key
    pub pubkey: Pubkey,
    /// Whether the account is a signer
    pub is_signer: bool,
    /// Whether the account is writable
    pub is_writable: bool,
    /// Account role in the instruction
    pub role: AccountRole,
}

/// Role of an account in an instruction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountRole {
    /// Program executable account
    Program,
    /// Data account being read
    Readonly,
    /// Data account being written
    Writable,
    /// Payer account
    Payer,
    /// System account
    System,
}

impl Program {
    /// Create a new program
    pub fn new(
        program_id: Pubkey,
        name: String,
        version: String,
        description: String,
        data: Vec<u8>,
        authority: Option<Pubkey>,
    ) -> Self {
        Self {
            program_id,
            name,
            version,
            description,
            authority,
            data,
            accounts: HashMap::new(),
            status: ProgramStatus::Deploying,
            deployed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Add an account to the program
    pub fn add_account(&mut self, account: ProgramAccount) {
        self.accounts.insert(account.pubkey, account);
    }

    /// Get an account by pubkey
    pub fn get_account(&self, pubkey: &Pubkey) -> Option<&ProgramAccount> {
        self.accounts.get(pubkey)
    }

    /// Mark program as deployed
    pub fn mark_deployed(&mut self) {
        self.status = ProgramStatus::Deployed;
    }

    /// Get total size of all accounts
    pub fn total_accounts_size(&self) -> usize {
        self.accounts.values().map(|acc| acc.size).sum()
    }

    /// Get total lamports across all accounts
    pub fn total_lamports(&self) -> u64 {
        self.accounts.values().map(|acc| acc.lamports).sum()
    }
}

impl ProgramAccount {
    /// Create a new program account
    pub fn new(
        pubkey: Pubkey,
        owner: Pubkey,
        data: Vec<u8>,
        lamports: u64,
        executable: bool,
        metadata: AccountMetadata,
    ) -> Self {
        let size = data.len();
        Self {
            pubkey,
            owner,
            data,
            lamports,
            executable,
            size,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata,
        }
    }

    /// Update account data
    pub fn update_data(&mut self, new_data: Vec<u8>) {
        self.data = new_data;
        self.size = self.data.len();
    }

    /// Add lamports to the account
    pub fn add_lamports(&mut self, amount: u64) {
        self.lamports += amount;
    }

    /// Subtract lamports from the account
    pub fn subtract_lamports(&mut self, amount: u64) -> Result<(), String> {
        if self.lamports >= amount {
            self.lamports -= amount;
            Ok(())
        } else {
            Err("Insufficient lamports".to_string())
        }
    }

    /// Check if account is a Program Derived Address (PDA)
    pub fn is_pda(&self) -> bool {
        !self.metadata.seeds.is_empty()
    }
}

impl ProgramDeployment {
    /// Create a new program deployment
    pub fn new(program: Program, config: DeploymentConfig) -> Self {
        Self {
            program,
            config,
            required_signatures: Vec::new(),
            fee: 1_000_000, // Default 0.001 SOL
        }
    }

    /// Add a required signature
    pub fn add_required_signature(&mut self, pubkey: Pubkey) {
        if !self.required_signatures.contains(&pubkey) {
            self.required_signatures.push(pubkey);
        }
    }

    /// Set deployment fee
    pub fn set_fee(&mut self, fee: u64) {
        self.fee = fee;
    }
}

impl CreateAccountRequest {
    /// Create a new account creation request
    pub fn new(
        account: ProgramAccount,
        program_id: Pubkey,
        payer: Pubkey,
        params: CreateAccountParams,
    ) -> Self {
        Self {
            account,
            program_id,
            payer,
            params,
        }
    }

    /// Calculate total cost in lamports
    pub fn total_cost(&self) -> u64 {
        self.params.lamports + self.account.lamports
    }
}

impl ProgramInstruction {
    /// Create a new program instruction
    pub fn new(
        program_id: Pubkey,
        accounts: Vec<InstructionAccount>,
        data: Vec<u8>,
        instruction_id: u8,
    ) -> Self {
        Self {
            program_id,
            accounts,
            data,
            instruction_id,
        }
    }

    /// Get writable accounts
    pub fn writable_accounts(&self) -> Vec<&InstructionAccount> {
        self.accounts.iter().filter(|acc| acc.is_writable).collect()
    }

    /// Get signer accounts
    pub fn signer_accounts(&self) -> Vec<&InstructionAccount> {
        self.accounts.iter().filter(|acc| acc.is_signer).collect()
    }
}

impl InstructionAccount {
    /// Create a new instruction account
    pub fn new(pubkey: Pubkey, is_signer: bool, is_writable: bool, role: AccountRole) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable,
            role,
        }
    }

    /// Create a program account
    pub fn program(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            is_signer: false,
            is_writable: false,
            role: AccountRole::Program,
        }
    }

    /// Create a writable data account
    pub fn writable(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            is_signer: false,
            is_writable: true,
            role: AccountRole::Writable,
        }
    }

    /// Create a readonly data account
    pub fn readonly(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            is_signer: false,
            is_writable: false,
            role: AccountRole::Readonly,
        }
    }

    /// Create a signer account
    pub fn signer(pubkey: Pubkey, is_writable: bool) -> Self {
        Self {
            pubkey,
            is_signer: true,
            is_writable,
            role: AccountRole::Payer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::constants::SYSTEM_PROGRAM_ID;

    #[test]
    fn test_program_creation() {
        let program_id = Pubkey::new([1; 32]);
        let program = Program::new(
            program_id,
            "Test Program".to_string(),
            "1.0.0".to_string(),
            "A test program".to_string(),
            vec![1, 2, 3, 4],
            Some(SYSTEM_PROGRAM_ID),
        );

        assert_eq!(program.program_id, program_id);
        assert_eq!(program.name, "Test Program");
        assert_eq!(program.status, ProgramStatus::Deploying);
        assert!(program.accounts.is_empty());
    }

    #[test]
    fn test_program_account_creation() {
        let pubkey = Pubkey::new([2; 32]);
        let owner = Pubkey::new([3; 32]);
        let metadata = AccountMetadata {
            name: "Test Account".to_string(),
            account_type: "data".to_string(),
            description: "A test account".to_string(),
            mutable: true,
            seeds: vec![],
        };

        let account = ProgramAccount::new(pubkey, owner, vec![5, 6, 7, 8], 1000, false, metadata);

        assert_eq!(account.pubkey, pubkey);
        assert_eq!(account.owner, owner);
        assert_eq!(account.lamports, 1000);
        assert_eq!(account.size, 4);
        assert!(!account.executable);
    }

    #[test]
    fn test_program_instruction() {
        let program_id = Pubkey::new([4; 32]);
        let account1 = InstructionAccount::readonly(Pubkey::new([5; 32]));
        let account2 = InstructionAccount::writable(Pubkey::new([6; 32]));

        let instruction =
            ProgramInstruction::new(program_id, vec![account1, account2], vec![9, 10, 11], 1);

        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 2);
        assert_eq!(instruction.instruction_id, 1);
        assert_eq!(instruction.writable_accounts().len(), 1);
    }

    #[test]
    fn test_account_metadata() {
        let metadata = AccountMetadata {
            name: "PDA Account".to_string(),
            account_type: "pda".to_string(),
            description: "Program derived address".to_string(),
            mutable: false,
            seeds: vec![vec![1, 2], vec![3, 4]],
        };

        assert_eq!(metadata.name, "PDA Account");
        assert_eq!(metadata.account_type, "pda");
        assert!(!metadata.mutable);
        assert_eq!(metadata.seeds.len(), 2);
    }
}

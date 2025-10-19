//! Domain layer - Core business logic and entities
//!
//! This module contains the core domain types and business logic for the Solana
//! library, following Domain-Driven Design principles.

pub mod programs;
pub mod types;

// Re-export commonly used domain types
pub use programs::{
    AccountMetadata, AccountRole, CreateAccountParams, CreateAccountRequest, DeploymentConfig,
    InstructionAccount, Program, ProgramAccount, ProgramDeployment, ProgramInstruction,
    ProgramStatus,
};
pub use types::{Hash, HashError, Pubkey, PubkeyError, Signature, SignatureError};

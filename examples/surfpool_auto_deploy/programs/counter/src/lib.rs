//! Anchor Counter Program
//!
//! A simple counter program that demonstrates auto-deployment with surfpool.
//! This program will be automatically deployed when surfpool starts.

use anchor_lang::prelude::*;

declare_id!("CounterProgram111111111111111111111111111111");

#[program]
pub mod counter {
    use super::*;

    /// Initialize a new counter account
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, bump: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        counter.bump = bump;
        counter.created_at = Clock::get()?.unix_timestamp;
        counter.last_updated = Clock::get()?.unix_timestamp;

        msg!("Counter initialized with authority: {}", authority);
        Ok(())
    }

    /// Increment the counter by 1
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        counter.count += 1;
        counter.last_updated = Clock::get()?.unix_timestamp;

        msg!("Counter incremented to: {}", counter.count);
        Ok(())
    }

    /// Decrement the counter by 1 (cannot go below 0)
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        require!(counter.count > 0, ErrorCode::Underflow);

        counter.count -= 1;
        counter.last_updated = Clock::get()?.unix_timestamp;

        msg!("Counter decremented to: {}", counter.count);
        Ok(())
    }

    /// Reset counter to zero
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        counter.count = 0;
        counter.last_updated = Clock::get()?.unix_timestamp;

        msg!("Counter reset to zero");
        Ok(())
    }

    /// Set counter to specific value
    pub fn set(ctx: Context<Set>, value: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(
            counter.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        counter.count = value;
        counter.last_updated = Clock::get()?.unix_timestamp;

        msg!("Counter set to: {}", value);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(authority: Pubkey, bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + CounterAccount::INIT_SPACE,
        seeds = [b"counter", authority.as_ref()],
        bump
    )]
    pub counter: Account<'info, CounterAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
        has_one = authority
    )]
    pub counter: Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(
        mut,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
        has_one = authority
    )]
    pub counter: Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
        has_one = authority
    )]
    pub counter: Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(
        mut,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
        has_one = authority
    )]
    pub counter: Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct CounterAccount {
    pub authority: Pubkey,
    pub count: u64,
    pub created_at: i64,
    pub last_updated: i64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Only the account authority can perform this action")]
    Unauthorized,

    #[msg("Underflow: Counter cannot go below zero")]
    Underflow,

    #[msg("Invalid timestamp: Created time cannot be after last updated")]
    InvalidTimestamp,
}

// Additional helper functions for gloo_solana integration
impl CounterAccount {
    /// Get the PDA seeds for this account
    pub fn seeds(authority: &Pubkey) -> Vec<&[u8]> {
        vec![b"counter", authority.as_ref()]
    }

    /// Get the discriminator for account serialization
    pub fn discriminator() -> [u8; 8] {
        use anchor_lang::Discriminator;
        let mut disc = [0u8; 8];
        disc.copy_from_slice(Self::DISCRIMINATOR);
        disc
    }

    /// Validate the account structure
    pub fn validate(&self) -> Result<()> {
        require!(
            self.created_at <= self.last_updated,
            ErrorCode::InvalidTimestamp
        );
        Ok(())
    }
}

// Integration helpers for gloo_solana auto-deployment
pub mod auto_deploy {
    use super::*;

    /// Create instruction data for Initialize instruction
    pub fn initialize_instruction_data(authority: Pubkey, bump: u8) -> Vec<u8> {
        let mut data = Vec::new();

        // Instruction discriminator (8 bytes) - use a placeholder for initialize
        data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // Authority (32 bytes)
        data.extend_from_slice(authority.as_ref());

        // Bump (1 byte)
        data.push(bump);

        data
    }

    /// Create instruction data for Increment instruction
    pub fn increment_instruction_data() -> Vec<u8> {
        let mut data = Vec::new();

        // Instruction discriminator (8 bytes)
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        data
    }

    /// Create instruction data for Decrement instruction
    pub fn decrement_instruction_data() -> Vec<u8> {
        let mut data = Vec::new();

        // Instruction discriminator (8 bytes)
        data.extend_from_slice(&[0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        data
    }

    /// Create instruction data for Reset instruction
    pub fn reset_instruction_data() -> Vec<u8> {
        let mut data = Vec::new();

        // Instruction discriminator (8 bytes)
        data.extend_from_slice(&[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        data
    }

    /// Create instruction data for Set instruction
    pub fn set_instruction_data(value: u64) -> Vec<u8> {
        let mut data = Vec::new();

        // Instruction discriminator (8 bytes)
        data.extend_from_slice(&[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // Value (8 bytes)
        data.extend_from_slice(&value.to_le_bytes());

        data
    }
}

// Auto-deployment metadata
pub mod deployment {
    use super::*;

    /// Program metadata for auto-deployment
    pub const PROGRAM_NAME: &str = "counter";
    pub const PROGRAM_VERSION: &str = "1.0.0";
    pub const PROGRAM_DESCRIPTION: &str = "Anchor counter program for auto-deployment";

    /// Get program features for deployment
    pub fn get_features() -> Vec<&'static str> {
        vec![
            "Initialize counter account with authority",
            "Increment counter value",
            "Decrement counter value",
            "Reset counter to zero",
            "Set counter to specific value",
            "Anchor-compatible account discriminators",
            "PDA support with bump seeds",
            "Authority-based access control",
        ]
    }

    /// Deployment configuration structure
    pub struct DeploymentConfigStruct {
        pub skip_preflight: bool,
        pub max_compute_units: Option<u32>,
        pub priority_fee: Option<u32>,
        pub commitment: String,
    }

    /// Get deployment configuration
    pub fn get_deployment_config() -> DeploymentConfigStruct {
        DeploymentConfigStruct {
            skip_preflight: true,
            max_compute_units: Some(200_000),
            priority_fee: Some(1000),
            commitment: "confirmed".to_string(),
        }
    }
}

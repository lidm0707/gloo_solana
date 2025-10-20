//! Core domain types for Solana primitives
//!
//! This module defines the fundamental types used throughout the Solana gloo_net library,
//! providing WASM-compatible implementations of Solana's core data structures.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A Solana public key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pubkey([u8; 32]);

impl Serialize for Pubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base58())
    }
}

impl<'de> Deserialize<'de> for Pubkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Pubkey::from_base58(&s).map_err(serde::de::Error::custom)
    }
}

impl Pubkey {
    /// Create a new pubkey from a 32-byte array
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create a pubkey from a base58 string
    pub fn from_base58(s: &str) -> Result<Self, PubkeyError> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| PubkeyError::InvalidBase58)?;

        if bytes.len() != 32 {
            return Err(PubkeyError::InvalidLength);
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }

    /// Convert to base58 string
    pub fn to_base58(&self) -> String {
        bs58::encode(self.0).into_string()
    }

    /// Create a new random pubkey (placeholder - would need proper WASM-compatible RNG)
    pub fn new_unique() -> Self {
        // In a real implementation, this would use a cryptographically secure RNG
        // For now, return a zero pubkey as placeholder
        Self([0u8; 32])
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl std::str::FromStr for Pubkey {
    type Err = PubkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pubkey::from_base58(s)
    }
}

/// A Solana signature
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signature([u8; 64]);

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base58())
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Signature::from_base58(&s).map_err(serde::de::Error::custom)
    }
}

impl Signature {
    /// Create a new signature from a 64-byte array
    pub fn new(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Create a signature from a base58 string
    pub fn from_base58(s: &str) -> Result<Self, SignatureError> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| SignatureError::InvalidBase58)?;

        if bytes.len() != 64 {
            return Err(SignatureError::InvalidLength);
        }

        let mut array = [0u8; 64];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }

    /// Convert to base58 string
    pub fn to_base58(&self) -> String {
        bs58::encode(self.0).into_string()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

/// A Solana hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base58())
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Hash::from_base58(&s).map_err(serde::de::Error::custom)
    }
}

impl Hash {
    /// Create a new hash from a 32-byte array
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create a hash from a base58 string
    pub fn from_base58(s: &str) -> Result<Self, HashError> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| HashError::InvalidBase58)?;

        if bytes.len() != 32 {
            return Err(HashError::InvalidLength);
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }

    /// Convert to base58 string
    pub fn to_base58(&self) -> String {
        bs58::encode(self.0).into_string()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

/// Errors related to pubkey operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum PubkeyError {
    #[error("Invalid base58 encoding")]
    InvalidBase58,
    #[error("Invalid pubkey length: expected 32 bytes")]
    InvalidLength,
}

/// Errors related to signature operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum SignatureError {
    #[error("Invalid base58 encoding")]
    InvalidBase58,
    #[error("Invalid signature length: expected 64 bytes")]
    InvalidLength,
}

/// Errors related to hash operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum HashError {
    #[error("Invalid base58 encoding")]
    InvalidBase58,
    #[error("Invalid hash length: expected 32 bytes")]
    InvalidLength,
}

/// Common constants
pub mod constants {
    use super::Pubkey;

    /// The system program ID
    pub const SYSTEM_PROGRAM_ID: Pubkey = Pubkey([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    /// The rent sysvar ID
    pub const SYSVAR_RENT_ID: Pubkey = Pubkey([
        6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133,
        237, 95, 91, 55, 145, 58, 11, 40, 209, 98, 225, 236, 9,
    ]);

    /// The clock sysvar ID
    pub const SYSVAR_CLOCK_ID: Pubkey = Pubkey([
        6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133,
        237, 95, 91, 55, 145, 58, 11, 40, 209, 98, 225, 236, 10,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_base58_roundtrip() {
        let pubkey = Pubkey::new([1; 32]);
        let base58 = pubkey.to_base58();
        let decoded = Pubkey::from_base58(&base58).unwrap();
        assert_eq!(pubkey, decoded);
    }

    #[test]
    fn test_signature_base58_roundtrip() {
        let signature = Signature::new([1; 64]);
        let base58 = signature.to_base58();
        let decoded = Signature::from_base58(&base58).unwrap();
        assert_eq!(signature, decoded);
    }

    #[test]
    fn test_hash_base58_roundtrip() {
        let hash = Hash::new([1; 32]);
        let base58 = hash.to_base58();
        let decoded = Hash::from_base58(&base58).unwrap();
        assert_eq!(hash, decoded);
    }
}

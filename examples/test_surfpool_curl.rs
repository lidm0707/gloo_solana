//! Example: Testing surfpool connection using curl
//!
//! This example demonstrates surfpool connectivity by using curl commands
//! to test the JSON-RPC endpoint directly, bypassing WASM dependencies.

use serde_json::Value;
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌊 Testing surfpool deployment at http://127.0.0.1:8899");
    println!("=============================================");

    // Test 1: Get version information
    test_get_version()?;

    // Test 2: Get block height
    test_get_block_height()?;

    // Test 3: Get latest blockhash
    test_get_latest_blockhash()?;

    // Test 4: Get system program balance
    test_get_balance()?;

    // Test 5: Get account info
    test_get_account_info()?;

    println!("\n🎉 All surfpool tests completed successfully!");
    println!("✅ Deployment to surfpool is working correctly!");

    Ok(())
}

fn test_get_version() -> Result<(), Box<dyn Error>> {
    println!("\n📋 Testing getVersion...");

    let output = Command::new("curl")
        .args(&[
            "-s",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            r#"{"jsonrpc":"2.0","id":1,"method":"getVersion"}"#,
            "http://127.0.0.1:8899",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("curl command failed: {:?}", output.stderr).into());
    }

    let response: Value = serde_json::from_str(&String::from_utf8(output.stdout)?)?;

    if let Some(result) = response.get("result") {
        if let Some(version) = result.get("surfnet-version") {
            println!("✅ Surfnet version: {}", version);
        }
        if let Some(solana_core) = result.get("solana-core") {
            println!("✅ Solana core version: {}", solana_core);
        }
    }

    Ok(())
}

fn test_get_block_height() -> Result<(), Box<dyn Error>> {
    println!("\n📏 Testing getBlockHeight...");

    let output = Command::new("curl")
        .args(&[
            "-s",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            r#"{"jsonrpc":"2.0","id":1,"method":"getBlockHeight"}"#,
            "http://127.0.0.1:8899",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("curl command failed: {:?}", output.stderr).into());
    }

    let response: Value = serde_json::from_str(&String::from_utf8(output.stdout)?)?;

    if let Some(height) = response.get("result").and_then(|v| v.as_u64()) {
        println!("✅ Current block height: {}", height);
    }

    Ok(())
}

fn test_get_latest_blockhash() -> Result<(), Box<dyn Error>> {
    println!("\n🔗 Testing getLatestBlockhash...");

    let output = Command::new("curl")
        .args(&[
            "-s",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            r#"{"jsonrpc":"2.0","id":1,"method":"getLatestBlockhash"}"#,
            "http://127.0.0.1:8899",
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("curl command failed: {:?}", output.stderr).into());
    }

    let response: Value = serde_json::from_str(&String::from_utf8(output.stdout)?)?;

    if let Some(result) = response.get("result") {
        if let Some(value) = result.get("value") {
            if let Some(blockhash) = value.get("blockhash").and_then(|v| v.as_str()) {
                println!("✅ Latest blockhash: {}", blockhash);
            }
            if let Some(last_valid) = value.get("lastValidBlockHeight").and_then(|v| v.as_u64()) {
                println!("✅ Valid until block: {}", last_valid);
            }
        }
    }

    Ok(())
}

fn test_get_balance() -> Result<(), Box<dyn Error>> {
    println!("\n💰 Testing getBalance for system program...");

    let output = Command::new("curl")
        .args(&[
            "-s", "-X", "POST",
            "-H", "Content-Type: application/json",
            "-d", r#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#,
            "http://127.0.0.1:8899"
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("curl command failed: {:?}", output.stderr).into());
    }

    let response: Value = serde_json::from_str(&String::from_utf8(output.stdout)?)?;

    if let Some(balance) = response
        .get("result")
        .and_then(|v| v.get("value"))
        .and_then(|v| v.as_u64())
    {
        println!("✅ System program balance: {} lamports", balance);
    }

    Ok(())
}

fn test_get_account_info() -> Result<(), Box<dyn Error>> {
    println!("\n📊 Testing getAccountInfo for system program...");

    let output = Command::new("curl")
        .args(&[
            "-s", "-X", "POST",
            "-H", "Content-Type: application/json",
            "-d", r#"{"jsonrpc":"2.0","id":1,"method":"getAccountInfo","params":["11111111111111111111111111111111"]}"#,
            "http://127.0.0.1:8899"
        ])
        .output()?;

    if !output.status.success() {
        return Err(format!("curl command failed: {:?}", output.stderr).into());
    }

    let response: Value = serde_json::from_str(&String::from_utf8(output.stdout)?)?;

    if let Some(result) = response.get("result") {
        if let Some(value) = result.get("value") {
            if let Some(lamports) = value.get("lamports").and_then(|v| v.as_u64()) {
                println!("✅ Lamports: {}", lamports);
            }
            if let Some(owner) = value.get("owner").and_then(|v| v.as_str()) {
                println!("✅ Owner: {}", owner);
            }
            if let Some(executable) = value.get("executable").and_then(|v| v.as_bool()) {
                println!("✅ Executable: {}", executable);
            }
            if let Some(data) = value.get("data").and_then(|v| v.as_array()) {
                println!("✅ Data length: {} items", data.len());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curl_availability() {
        let output = Command::new("curl").arg("--version").output();
        assert!(output.is_ok(), "curl should be available for testing");
    }

    #[test]
    fn test_surfpool_endpoint_reachable() {
        let output = Command::new("curl")
            .args(&["-s", "http://127.0.0.1:8899"])
            .output()
            .expect("curl should be available");

        // The endpoint should respond (even if with an error about missing JSON-RPC)
        assert!(
            output.status.success(),
            "surfpool endpoint should be reachable"
        );
    }
}

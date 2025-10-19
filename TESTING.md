# Testing Guide

This guide covers how to test the `gloo_solana` library, including unit tests, integration tests, and surfpool testing.

## 🧪 Overview

The `gloo_solana` library includes several types of tests:

- **Unit Tests**: Test individual functions and types
- **Integration Tests**: Test component interactions
- **Surfpool Tests**: Test against a local Solana network
- **WASM Tests**: Test in browser environments

## 🚀 Quick Start

### Run All Tests

```bash
# Run unit tests
cargo test

# Run unit tests with output
cargo test -- --nocapture

# Run specific test module
cargo test domain::types::tests

# Run specific test
cargo test test_pubkey_base58_roundtrip
```

### Run Examples

```bash
# Run basic functionality test
cargo run --example basic_test

# Run surfpool test (requires surfpool running)
cargo run --example test_surfpool

# Run Dioxus example
cargo run --example dioxus_app --features dioxus
```

## 🌊 Surfpool Testing

Surfpool is a local Solana network simulator that allows you to test Solana interactions without using real tokens.

### Step 1: Install Surfpool

```bash
# Install surfpool using cargo
cargo install surfpool

# Or download from GitHub releases
# https://github.com/solana-labs/solana/releases
```

### Step 2: Start Surfpool

```bash
# Start surfpool in the background
surfpool start

# Or run in a separate terminal
surfpool start --log-level info

# Start with custom config
surfpool start --config surfpool-config.yaml
```

### Step 3: Verify Surfpool is Running

```bash
# Test surfpool connection
curl -X POST -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
     http://127.0.0.1:8899

# Expected response:
# {"jsonrpc":"2.0","result":{"surfnet-version":"0.10.7","solana-core":"2.3.8","feature-set":2255652435},"id":1}
```

### Step 4: Run Surfpool Tests

```bash
# Run the surfpool test example
cargo run --example test_surfpool

# Run unit tests that work with surfpool
cargo test --features "test-surfpool"
```

### Step 5: Stop Surfpool

```bash
# Stop surfpool when done
surfpool stop
```

## 🧪 Unit Tests

### Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run tests in a specific module
cargo test domain::types

# Run a specific test
cargo test test_pubkey_base58_roundtrip
```

### Available Unit Tests

- **Domain Types**: Test Pubkey, Signature, Hash serialization
- **HTTP Client**: Test HTTP client functionality
- **RPC Client**: Test JSON-RPC client operations
- **Application Services**: Test business logic services

### Example Test Output

```bash
$ cargo test

running 12 tests
test domain::types::tests::test_pubkey_base58_roundtrip ... ok
test domain::types::tests::test_signature_base58_roundtrip ... ok
test domain::types::tests::test_hash_base58_roundtrip ... ok
test infrastructure::http::tests::test_http_client_builder ... ok
test infrastructure::rpc::tests::test_network_endpoints ... ok
test infrastructure::rpc::tests::test_rpc_client_builder ... ok
test infrastructure::rpc::tests::test_rpc_request_serialization ... ok
test tests::test_create_clients ... ok
test tests::test_network_endpoints ... ok
test tests::test_version ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 🌐 WASM Testing

Since `gloo_solana` is designed for WASM environments, you'll want to test in a browser.

### Install wasm-pack

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or using cargo
cargo install wasm-pack
```

### Run WASM Tests

```bash
# Run WASM tests in headless browser
wasm-pack test --headless --firefox

# Run in Chrome
wasm-pack test --headless --chrome

# Run with debug output
wasm-pack test --headless --firefox --debug
```

### Build for Web

```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Build for bundler (webpack, etc.)
wasm-pack build --target bundler --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg
```

## 📝 Integration Tests

### Creating Integration Tests

Create tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use gloo_solana::{RpcClientBuilder, surfpool_network};

#[tokio::test]
async fn test_surfpool_integration() {
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .build();
    
    // Test actual surfpool connection
    let result = client.get_block_height().await;
    assert!(result.is_ok());
}
```

### Running Integration Tests

```bash
# Run all tests including integration tests
cargo test --test integration_test

# Run only integration tests
cargo test --test "*"

# Run with specific test name
cargo test --test integration_test test_surfpool_integration
```

## 🔧 Test Configuration

### Cargo.toml Test Configuration

```toml
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
log = "0.4"
console_log = "1.0"
wasm-bindgen-test = "0.3"

[[test]]
name = "surfpool_integration"
required-features = ["test-surfpool"]
```

### Environment Variables

```bash
# Set log level for tests
export RUST_LOG=debug

# Run tests with logging
RUST_LOG=debug cargo test -- --nocapture

# Set surfpool endpoint
export SURFPOOL_ENDPOINT=http://127.0.0.1:8899

# Run with custom endpoint
SURFPOOL_ENDPOINT=http://localhost:8899 cargo run --example test_surfpool
```

## 🐛 Debugging Tests

### Common Issues

1. **Surfpool not running**
   ```bash
   # Check if surfpool is running
   curl http://127.0.0.1:8899
   
   # Start surfpool
   surfpool start
   ```

2. **WASM compilation errors**
   ```bash
   # Clear build cache
   cargo clean
   
   # Rebuild
   cargo build --target wasm32-unknown-unknown
   ```

3. **Network connectivity issues**
   ```bash
   # Test network connection
   curl -X POST -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
        http://127.0.0.1:8899
   ```

### Debug Output

```bash
# Run tests with debug output
cargo test -- --nocapture

# Run with specific log level
RUST_LOG=debug cargo test -- --nocapture

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo test

# Run with full backtrace
RUST_BACKTRACE=full cargo test
```

## 📊 Test Coverage

### Install tarpaulin

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin
```

### Generate Coverage Report

```bash
# Generate coverage report
cargo tarpaulin --out Html

# Generate coverage for specific module
cargo tarpaulin --lib -- -- domain::types

# Generate coverage and open in browser
cargo tarpaulin --out Html && open tarpaulin-report.html
```

## 🔄 Continuous Integration

### GitHub Actions Example

```yaml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test --verbose
      
    - name: Run WASM tests
      run: wasm-pack test --headless --firefox
      
  surfpool-test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install surfpool
      run: cargo install surfpool
      
    - name: Start surfpool
      run: surfpool start &
      
    - name: Wait for surfpool
      run: sleep 10
      
    - name: Run surfpool tests
      run: cargo run --example test_surfpool
      
    - name: Stop surfpool
      run: surfpool stop
```

## 📚 Test Examples

### Basic Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pubkey_creation() {
        let pubkey = Pubkey::new([1; 32]);
        assert_eq!(pubkey.as_bytes(), &[1; 32]);
    }
    
    #[test]
    fn test_base58_roundtrip() {
        let original = Pubkey::new([42; 32]);
        let encoded = original.to_base58();
        let decoded = Pubkey::from_base58(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}
```

### Async Test Example

```rust
#[tokio::test]
async fn test_rpc_client() {
    let client = RpcClientBuilder::new("http://127.0.0.1:8899").build();
    
    // Test basic connectivity
    let result = client.get_block_height().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_account_operations() {
    let client = create_test_client();
    let pubkey = SYSTEM_PROGRAM_ID;
    
    // Test account info
    let account = client.get_account_info(&pubkey).await.unwrap();
    assert!(account.is_some());
    
    // Test balance
    let balance = client.get_balance(&pubkey).await.unwrap();
    assert!(balance >= 0);
}
```

## 🎯 Best Practices

1. **Test Naming**: Use descriptive test names
2. **Test Organization**: Group related tests in modules
3. **Mock Dependencies**: Use mocks for external dependencies
4. **Async Testing**: Use `#[tokio::test]` for async tests
5. **Error Testing**: Test both success and error cases
6. **Cleanup**: Clean up resources in tests
7. **Isolation**: Ensure tests don't depend on each other

## 🚀 Quick Test Commands

```bash
# Quick test run
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run surfpool test (ensure surfpool is running)
cargo run --example test_surfpool

# Run basic functionality test
cargo run --example basic_test

# Run WASM tests
wasm-pack test --headless --firefox
```

Happy testing! 🧪
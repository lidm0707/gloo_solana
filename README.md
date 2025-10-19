# gloo_solana

🌊 A WASM-compatible Solana SDK rewrite using `gloo_net` for web environments.

## Overview

`gloo_solana` provides a complete rewrite of Solana SDK functionality designed specifically for WASM environments. Instead of using direct TCP connections, it communicates with Solana networks via HTTP JSON-RPC calls, making it perfect for web applications, Dioxus frontends, and browser-based dApps.

## Features

- 🌐 **WASM-First Design**: Built from the ground up for browser environments
- 🔗 **HTTP-Based Communication**: Uses `gloo_net` for JSON-RPC over HTTP
- 🏗️ **DDD Architecture**: Domain-Driven Design with SOLID principles
- ⚡ **Single-Threaded Compatible**: Works with WASM's single-threaded execution model
- 🌊 **Surfpool Support**: Full support for local development with surfpool (simnet)
- 🔀 **Multi-Network**: Support for mainnet, devnet, testnet, and custom endpoints
- 🎯 **Dioxus Integration**: Seamless integration with Dioxus web framework
- 🛡️ **Type Safe**: Full Rust type safety with comprehensive error handling

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gloo_solana = "0.1.0"
gloo-net = { version = "0.6", features = ["http"] }
serde = { version = "1.0", features = ["derive"] }
```

For Dioxus integration:

```toml
[dependencies]
gloo_solana = { version = "0.1.0", features = ["dioxus"] }
dioxus = "0.6"
```

### Basic Usage

```rust
use gloo_solana::{RpcClientBuilder, surfpool_network, CommitmentLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create RPC client for surfpool (local development)
    let client = RpcClientBuilder::new(surfpool_network().endpoint())
        .commitment(CommitmentLevel::Confirmed)
        .build();

    // Get account balance
    let balance = client.get_balance(&system_program_id()).await?;
    println!("System program balance: {} lamports", balance);

    Ok(())
}
```

## Core Concepts

### Network Configuration

```rust
use gloo_solana::{Network, surfpool_network};

// Built-in networks
let mainnet_client = RpcClientBuilder::new(Network::Mainnet.endpoint()).build();
let devnet_client = RpcClientBuilder::new(Network::Devnet.endpoint()).build();
let testnet_client = RpcClientBuilder::new(Network::Testnet.endpoint()).build();

// Custom endpoints
let surfpool_client = RpcClientBuilder::new(surfpool_network().endpoint()).build();
let custom_client = RpcClientBuilder::new("https://custom-rpc.example.com").build();
```

### Public Keys and Signatures

```rust
use gloo_solana::Pubkey;

// Create from base58 string
let pubkey = Pubkey::from_base58("11111111111111111111111111111111")?;

// Create from bytes
let bytes = [1u8; 32];
let pubkey = Pubkey::new(bytes);

// Convert back to base58
let base58_string = pubkey.to_string();
```

### RPC Operations

```rust
// Get account information
let account = client.get_account_info(&pubkey).await?;

// Get account balance
let balance = client.get_balance(&pubkey).await?;

// Get latest blockhash
let latest_blockhash = client.get_latest_blockhash().await?;

// Get multiple accounts
let accounts = client.get_multiple_accounts(&[pubkey1, pubkey2]).await?;

// Send transaction
let signature = client.send_transaction(&transaction_string).await?;
```

## Dioxus Integration

### Setting up the Provider

```rust
use dioxus::prelude::*;
use gloo_solana::dioxus_integration::*;

fn App(cx: Scope) -> Element {
    rsx! {
        SolanaProvider { network: surfpool_network(),
            // Your app components here
            BalanceDisplay { pubkey: system_program_id() }
        }
    }
}
```

### Using Hooks

```rust
use gloo_solana::dioxus_integration::*;

fn AccountExplorer(cx: Scope) -> Element {
    let solana_context = use_context::<SolanaContext>(cx).unwrap();
    
    // Get balance reactively
    let balance = use_balance(cx, &solana_context.client, pubkey);
    
    // Get account information reactively  
    let account_info = use_account_info(cx, &solana_context.client, pubkey);
    
    rsx! {
        match &*balance.read() {
            Some(Ok(lamports)) => rsx! { "Balance: {lamports} lamports" },
            Some(Err(e)) => rsx! { "Error: {e}" },
            None => rsx! { "Loading..." }
        }
    }
}
```

### Components

```rust
use gloo_solana::dioxus_integration::*;

fn NetworkSection(cx: Scope) -> Element {
    rsx! {
        // Network selector
        NetworkSelector {
            on_network_change: move |network: Network| {
                // Handle network change
            }
        }
        
        // Network information display
        NetworkInfo {}
        
        // Account balance display
        BalanceDisplay { pubkey: system_program_id() }
        
        // Detailed account information
        AccountInfo { pubkey: system_program_id() }
    }
}
```

## Architecture

The library follows Domain-Driven Design principles with clear separation of concerns:

```
src/
├── domain/           # Core business logic and entities
│   └── types/        # Pubkey, Signature, Hash, etc.
├── application/      # Use cases and application services  
│   └── services/     # AccountService, TransactionService, etc.
├── infrastructure/   # External integrations
│   ├── http/         # HTTP client abstraction
│   └── rpc/          # JSON-RPC client implementation
└── dioxus_integration/ # Dioxus components and hooks
```

## Surfpool Development

Surfpool is a local Solana network simulator perfect for development and testing.

### Starting Surfpool

```bash
# Start surfpool (requires surfpool to be installed)
surfpool start

# Verify it's running
curl -X POST -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"getVersion"}' \
     http://127.0.0.1:8899
```

### Using with gloo_solana

```rust
use gloo_solana::{surfpool_network, RpcClientBuilder};

let client = RpcClientBuilder::new(surfpool_network().endpoint())
    .commitment(CommitmentLevel::Confirmed)
    .build();

// All operations work the same as mainnet/devnet
let balance = client.get_balance(&pubkey).await?;
```

## Error Handling

The library provides comprehensive error handling:

```rust
use gloo_solana::{RpcError, HttpError};

match client.get_balance(&pubkey).await {
    Ok(balance) => println!("Balance: {}", balance),
    Err(RpcError::Http(HttpError::RequestError(msg))) => {
        eprintln!("Network error: {}", msg);
    }
    Err(RpcError::Http(HttpError::HttpStatusError { status, message })) => {
        eprintln!("HTTP {}: {}", status, message);
    }
    Err(RpcError::InvalidPubkey(e)) => {
        eprintln!("Invalid pubkey: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

- `basic_test.rs` - Core functionality testing
- `test_surfpool.rs` - Surfpool connection testing (WASM only)
- `dioxus_app.rs` - Complete Dioxus application example

## Testing

```bash
# Run basic functionality tests
cargo run --example basic_test

# Run unit tests
cargo test

# Run WASM tests (requires wasm-pack)
wasm-pack test --headless --firefox
```

## WASM Deployment

The library is designed specifically for WASM environments. To build for web:

```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Build for bundler (webpack, etc.)
wasm-pack build --target bundler --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg
```

## Dependencies

- `gloo-net` - HTTP client for WASM environments
- `serde` - Serialization/deserialization
- `wasm-bindgen` - WASM bindings
- `bs58` - Base58 encoding/decoding
- `base64` - Base64 encoding/decoding
- `thiserror` - Error handling

### Optional Dependencies

- `dioxus` - Web framework integration (feature flag)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under either of:

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Built on top of the excellent `gloo-net` library
- Inspired by the official Solana SDK
- Designed for the emerging WASM web ecosystem# gloo_solana

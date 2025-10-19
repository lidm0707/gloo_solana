# Examples Directory

This directory contains example programs that demonstrate how to use the `gloo_solana` library for interacting with Solana networks.

## 📁 Available Examples

### 🌊 [hello_surfpool_demo](./hello_surfpool_demo.rs)
A demonstration example that works without requiring network connections or WASM compilation.

**Features:**
- ✅ Creates accounts with custom data structures
- ✅ Demonstrates "hello surf" message handling
- ✅ Shows data serialization/deserialization
- ✅ Simulates multiple user accounts
- ✅ Works natively without surfpool running

**Run:**
```bash
cargo run --example hello_surfpool_demo
```

### 🌊 [hello_surfpool](./hello_surfpool/src/main.rs)
A full example that demonstrates real network operations (requires WASM environment).

**Features:**
- 🔌 Real HTTP connections to surfpool
- 🏦 Account operations on live network
- 📡 Network connectivity testing
- 🌐 Works with actual Solana data

**Requirements:**
- WASM environment
- Surfpool running locally

**Run:**
```bash
# Start surfpool first
surfpool start

# Run the example (requires WASM environment)
cargo run --example hello_surfpool
```

### 🔧 [basic_test](./basic_test.rs)
Basic functionality testing without network dependencies.

**Features:**
- 🧪 Core type testing (Pubkey, Signature, Hash)
- 🌐 Network configuration testing
- 🔧 RPC client creation testing
- 📊 Constants verification

**Run:**
```bash
cargo run --example basic_test
```

### 🎨 [dioxus_app](./dioxus_app.rs)
Complete Dioxus web application example (requires `dioxus` feature).

**Features:**
- 🖥️  Web-based Solana explorer
- 🎯 Interactive account management
- 🔄 Reactive UI components
- 📱 Responsive design

**Run:**
```bash
cargo run --example dioxus_app --features dioxus
```

### 🧪 [test_surfpool](./test_surfpool.rs)
Comprehensive testing example for surfpool integration.

**Features:**
- 🌊 Full surfpool testing suite
- 📡 Network connectivity verification
- 🔍 Account information testing
- ⚡ Performance benchmarks

**Run:**
```bash
# Requires surfpool running
cargo run --example test_surfpool
```

## 🚀 Getting Started

### Prerequisites

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Surfpool** (for network examples)
   ```bash
   cargo install surfpool
   ```

3. **Install wasm-pack** (for WASM examples)
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/lidm0707/gloo_solana.git
   cd gloo_solana
   ```

2. **Run the demo example** (works immediately)
   ```bash
   cargo run --example hello_surfpool_demo
   ```

3. **Start surfpool for network examples**
   ```bash
   surfpool start
   ```

4. **Run network examples**
   ```bash
   cargo run --example basic_test
   ```

## 📋 Example Categories

### 🟢 Beginner Examples
- `hello_surfpool_demo` - Start here, no dependencies
- `basic_test` - Core functionality testing

### 🟡 Intermediate Examples
- `test_surfpool` - Network testing
- `hello_surfpool` - Real network operations

### 🔴 Advanced Examples
- `dioxus_app` - Full web application

## 🛠️ Development Tips

### Running Tests
```bash
# Run all tests
cargo test

# Run example-specific tests
cargo test --example hello_surfpool_demo

# Run with output
cargo test -- --nocapture
```

### Building for WASM
```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg

# Build for bundlers
wasm-pack build --target bundler --out-dir pkg
```

### Debugging
```bash
# Run with debug output
RUST_LOG=debug cargo run --example hello_surfpool_demo

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test
```

## 📚 Learning Path

1. **Start with** `hello_surfpool_demo` to understand data structures
2. **Move to** `basic_test` to learn core operations
3. **Try** `test_surfpool` with a running surfpool instance
4. **Build** `dioxus_app` for web development
5. **Explore** `hello_surfpool` for advanced WASM usage

## 🔧 Common Issues

### WASM Compilation Errors
```bash
# Clear build cache
cargo clean

# Rebuild
cargo build --target wasm32-unknown-unknown
```

### Surfpool Connection Issues
```bash
# Check if surfpool is running
curl http://127.0.0.1:8899

# Start surfpool
surfpool start

# Check logs
surfpool logs
```

### Feature Flag Issues
```bash
# Enable dioxus feature
cargo run --example dioxus_app --features dioxus

# Check available features
cargo run --example dioxus_app --features dioxus --help
```

## 🤝 Contributing

To add new examples:

1. Create the example file in this directory
2. Add it to `Cargo.toml` under `[[example]]`
3. Include comprehensive comments and documentation
4. Add tests for the example functionality
5. Update this README with the new example

## 📄 Additional Resources

- [Main README](../README.md) - Library documentation
- [Testing Guide](../TESTING.md) - Comprehensive testing instructions
- [API Documentation](https://docs.rs/gloo_solana) - Rust API docs
- [Solana Documentation](https://docs.solana.com/) - Solana developer docs
- [Dioxus Documentation](https://dioxuslabs.com/) - Web framework docs

## 🎯 Example Goals

Each example aims to demonstrate:
- ✅ **Core Concepts**: fundamental Solana operations
- ✅ **Best Practices**: idiomatic Rust and gloo_solana usage
- ✅ **Real Scenarios**: practical use cases
- ✅ **Error Handling**: comprehensive error management
- ✅ **Documentation**: clear explanations and comments
- ✅ **Testing**: verified functionality

Happy coding! 🚀
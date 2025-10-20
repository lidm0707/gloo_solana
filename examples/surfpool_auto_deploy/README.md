# Surfpool Auto-Deploy with IDL Integration and Airdrop System

This example demonstrates a complete auto-deployment system that integrates directly with surfpool startup, providing automatic anchor program deployment, IDL generation, user airdrops, and solana program calls - all powered by gloo_net.

## 🎯 Overview

When you run `surfpool start`, this system automatically:

1. **📋 Reads Anchor.toml** - Discovers and parses your project's configuration
2. **🚀 Deploys Anchor Programs** - Automatically deploys all programs to surfpool
3. **📋 Generates IDL** - Creates Interface Definition Language files for program interaction
4. **🔧 Creates IDL Clients** - Generates program clients for calling instructions
5. **💰 Airdrops SOL** - Automatically funds users who will use the programs
6. **🎮 Demonstrates Program Calls** - Shows how to call programs using IDL

## 🌟 Key Features

- **🌊 Direct Surfpool Integration**: Just run `surfpool start` and everything happens automatically
- **📋 IDL-Based Development**: Generate IDL files for type-safe program interaction
- **💰 Smart Airdrops**: Fund users from local Solana CLI keypairs
- **🎮 Program Call Examples**: See how to call programs using generated IDL
- **🔧 gloo_net Powered**: Uses gloo_solana (powered by gloo_net) for WASM-compatible Solana development
- **📊 Real-time Monitoring**: Track deployment progress and results

## 📁 Project Structure

```
surfpool_auto_deploy/
├── Anchor.toml              # Enhanced configuration with IDL and airdrop settings
├── Cargo.toml               # Rust dependencies
├── README.md               # This file
├── src/
│   └── main.rs             # Main auto-deploy application with IDL integration
└── programs/
    └── counter/            # Anchor counter program example
        ├── Cargo.toml      # Program dependencies (anchor 0.31.1)
        └── src/
            └── lib.rs      # Counter program with IDL helpers
```

## 🚀 Quick Start

### Prerequisites

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Anchor CLI**:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   ```

3. **Install Surfpool**:
   ```bash
   cargo install surfpool
   ```

4. **Setup Solana CLI**:
   ```bash
   solana-keygen new --no-bip39 --silent
   ```

### Build and Run

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Start surfpool with full auto-integration**:
   ```bash
   cargo run --bin surfpool_deploy -- --start
   ```

3. **Deploy programs only** (if surfpool is already running):
   ```bash
   cargo run --bin surfpool_deploy -- --deploy-only
   ```

## 📋 Configuration

### Enhanced Anchor.toml

The `Anchor.toml` file includes comprehensive configuration for auto-deployment:

```toml
[features]
resolution = true
skip-lint = false

[programs.localnet]
counter = "CounterProgram111111111111111111111111111111"

[auto_deploy]
enabled = true
deploy_on_startup = true
generate_idl = true
create_program_clients = true
airdrop_users = true

[airdrop]
enabled = true
amount_per_user = 10000000000  # 10 SOL in lamports
skip_preflight = true

[[airdrop.users]]
keypair = "~/.config/solana/id.json"
name = "Local Developer"
role = "developer"
balance = 0
programs_to_use = ["counter"]

[idl]
output_dir = "target/idl"
include_accounts = true
include_instructions = true
include_types = true
format = "json"
```

### Key Configuration Sections

- **`[programs.localnet]`**: Define your anchor programs
- **`[auto_deploy]`**: Control auto-deployment features
- **`[airdrop]`**: Configure user funding
- **`[idl]`**: IDL generation settings

## 🔧 Command Line Options

```bash
surfpool_deploy [OPTIONS]

Options:
  -c, --config <FILE>     Path to Anchor.toml file [default: Anchor.toml]
  -s, --start             Start surfpool with full auto-integration
  -d, --deploy-only       Deploy programs only
  -a, --airdrop-only      Airdrop to users only
  -m, --demo-calls        Demonstrate IDL-based program calls
  -v, --verbose           Enable verbose logging
  -h, --help              Print help
  -V, --version           Print version
```

## 📊 Usage Examples

### Full Auto-Integration

Deploy everything when surfpool starts:

```bash
cargo run --bin surfpool_deploy -- --start
```

Output:
```
🌊 Starting Surfpool with Full Auto-Integration
════════════════════════════════════════════
🚀 Starting surfpool...
✅ Surfpool started successfully
⏳ Waiting for surfpool to be ready...
✅ Surfpool is ready! Blockhash: 5j7s8...
📦 Deploying anchor programs from Anchor.toml...
📋 Found 1 programs to deploy:
   • counter: CounterProgram111111111111111111111111111111

🚀 Deploying: counter
   ✅ Deployed successfully: CounterProgram111111111111111111111111111111

📋 Generating IDL for deployed programs...
   📋 Generated IDL for counter: "target/idl/counter.json"

🔧 Creating IDL-based program clients...
   🔧 Created IDL client for: counter

💰 Airdropping SOL to program users...
   💰 Airdropping 10 SOL to Local Developer (11111111111111111111111111111112)
   ✅ Airdropped! New balance: 10 SOL

🎮 Demonstrating program calls using IDL...
   🎮 Demonstrating calls for: counter
     📞 Calling initialize() using IDL...
       📋 Counter PDA: CounterPDA111111111111111111111111111111
       📋 Authority: 11111111111111111111111111111112
       📋 Instruction data: 8 bytes
       ✅ Initialize call completed
     📞 Calling increment() using IDL...
       📋 Instruction data: 8 bytes
       ✅ Increment call completed
     📞 Calling decrement() using IDL...
       📋 Instruction data: 8 bytes
       ✅ Decrement call completed

📊 Final Integration Status:
═════════════════════════════════
🌊 Surfpool: ✅ Running
📦 Deployed Programs:
   ✅ counter: CounterProgram111111111111111111111111111111
      📋 IDL: "target/idl/counter.json"

🔧 IDL Clients:
   • counter: 4 instructions

💰 User Balances:
   • Local Developer (11111111111111111111111111111112): 10 SOL

🎉 Full integration completed successfully!
🔗 Your programs are deployed and ready for dApp development!
```

### IDL Generation

The system automatically generates IDL files for your programs:

```json
{
  "version": "1.0.0",
  "name": "counter",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "counter",
          "isMut": true,
          "isSigner": false,
          "accountType": "CounterAccount"
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "accountType": "Signer"
        }
      ],
      "args": [
        {
          "name": "authority",
          "type": "PublicKey"
        }
      ]
    },
    {
      "name": "increment",
      "accounts": [
        {
          "name": "counter",
          "isMut": true,
          "isSigner": false,
          "accountType": "CounterAccount"
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true,
          "accountType": "Signer"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "CounterAccount",
      "type": "struct CounterAccount {\n    authority: Pubkey,\n    count: u64,\n    created_at: i64,\n    last_updated: i64,\n    bump: u8,\n}"
    }
  ]
}
```

## 🏗️ Architecture

### Core Components

1. **SurfpoolAutoDeployer**: Main auto-deployment engine
2. **AnchorIDL**: IDL structure generation and parsing
3. **IDLClient**: Type-safe program calling using IDL
4. **UserInfo**: User management for airdrops
5. **Anchor Programs**: Your Solana programs with IDL helpers

### Data Flow

```
Anchor.toml → Configuration Parser → Auto-Deployer → gloo_solana → Surfpool
     ↓                           ↓                    ↓
   Programs → IDL Generation → IDL Clients → Program Calls
     ↓                           ↓                    ↓
   Users → Airdrop System → Funded Users → dApp Ready
```

### Key Features

- **Configuration-Driven**: All settings in Anchor.toml
- **Error Handling**: Comprehensive error reporting and recovery
- **Progress Tracking**: Real-time deployment progress monitoring
- **Type Safety**: IDL-based program interaction
- **User Management**: Automatic user funding from keypairs

## 🎮 Program Interaction with IDL

### Initialize Counter Account

```rust
use anchor_lang::prelude::*;

// Load IDL client
let idl_client = IDLClient {
    program_id: Pubkey::from_base58("CounterProgram111111111111111111111111111111")?,
    idl: idl, // Loaded from target/idl/counter.json
    client: rpc_client,
};

// Find initialize instruction
let initialize_instruction = idl_client
    .idl
    .instructions
    .iter()
    .find(|inst| inst.name == "initialize")?;

// Build instruction data
let instruction_data = build_instruction_data(initialize_instruction, &authority.to_bytes())?;

// Get PDA for counter account
let counter_pda = find_counter_pda(&authority, &idl_client.program_id)?;

// Create transaction
let transaction = create_transaction_with_instruction(
    counter_pda,
    authority,
    idl_client.program_id,
    instruction_data,
);

// Send transaction
let signature = rpc_client.send_transaction(&transaction).await?;
```

### Using Generated IDL

The system generates IDL files that can be used with various tools:

```typescript
// TypeScript usage example
import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
import idl from "./target/idl/counter.json";

const program = new Program<Counter>(idl, programId, provider);

// Initialize counter
await program.methods
  .initialize(authority.publicKey)
  .accounts({
    counter: counterPDA,
    authority: authority.publicKey,
  })
  .rpc();
```

## 💰 Airdrop System

The airdrop system automatically funds users from local keypairs:

### Key Pair Integration

```toml
[[airdrop.users]]
keypair = "~/.config/solana/id.json"
name = "Local Developer"
role = "developer"
balance = 0
programs_to_use = ["counter"]
```

### Automatic Funding

- Reads keypair files from Solana CLI
- Extracts public keys from JSON keypair format
- Airdrops specified SOL amounts
- Updates user balances
- Tracks which programs each user can access

## 🔌 gloo_net Integration

This system showcases how `gloo_net` powers `gloo_solana` for WASM-compatible Solana development:

### HTTP JSON-RPC Communication

```rust
use gloo_solana::{RpcClientBuilder, surfpool_network, CommitmentLevel};

let client = RpcClientBuilder::new(surfpool_network().endpoint())
    .commitment(CommitmentLevel::Confirmed)
    .build();
```

### Program Deployment

```rust
use gloo_solana::{
    domain::programs::{Program, ProgramDeployment},
    constants::SYSTEM_PROGRAM_ID,
};

let program = Program::new(
    program_id,
    program_name,
    version,
    description,
    bytecode,
    Some(SYSTEM_PROGRAM_ID),
);

let deployment = ProgramDeployment::new(program, deployment_config);
```

### Real-time Monitoring

```rust
// Test connectivity
let blockhash = client.get_latest_blockhash().await?;
let block_height = client.get_block_height().await?;

// Deploy program
let deployed_id = deploy_program(deployment).await?;

// Verify deployment
let account_info = client.get_account_info(&deployed_id).await?;
```

## 🔄 Development Workflow

### 1. Project Setup

```bash
# Create anchor project with enhanced configuration
anchor init my_project
cd my_project

# Add auto-deploy configuration to Anchor.toml
# (See configuration section above)
```

### 2. Define Programs

```toml
[programs.localnet]
my_program = "MyProgram111111111111111111111111111111"
```

### 3. Auto-Deploy Everything

```bash
# This will:
# 1. Start surfpool
# 2. Deploy all programs
# 3. Generate IDL files
# 4. Create program clients
# 5. Airdrop to users
# 6. Demonstrate program calls
cargo run --bin surfpool_deploy -- --start
```

### 4. Build dApps with IDL

```typescript
// Use generated IDL in your dApp
import idl from "./target/idl/my_program.json";
import { Program, AnchorProvider } from "@project-serum/anchor";

const program = new Program<MyProgram>(idl, programId, provider);
```

## 🛠️ Advanced Features

### Multiple Programs

```toml
[programs.localnet]
counter = "CounterProgram111111111111111111111111111111"
vault = "VaultProgram111111111111111111111111111111"
storage = "StorageProgram111111111111111111111111111111"
```

### Custom Airdrop Amounts

```toml
[airdrop]
amount_per_user = 50000000000  # 50 SOL

[[airdrop.users]]
keypair = "~/.config/solana/id.json"
name = "Power User"
role = "developer"
```

### IDL Customization

```toml
[idl]
output_dir = "target/idl"
include_accounts = true
include_instructions = true
include_types = true
format = "json"
```

## 🧪 Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
# Test deployment without surfpool
cargo run --bin surfpool_deploy -- --deploy-only

# Test airdrop functionality
cargo run --bin surfpool_deploy -- --airdrop-only

# Test IDL program calls
cargo run --bin surfpool_deploy -- --demo-calls
```

### Program Tests

```bash
cd programs/counter
anchor test --skip-local-validator
```

## 🔍 Troubleshooting

### Common Issues

1. **"Surfpool failed to start"**
   - Install surfpool: `cargo install surfpool`
   - Check if surfpool is running: `surfpool status`

2. **"KeyPair file not found"**
   - Ensure Solana CLI is installed: `sh -c "$(curl -sSfL https://release.solana.com/v1.7.0/install)"`
   - Create keypair: `solana-keygen new`

3. **"Invalid pubkey"**
   - Check keypair file format
   - Ensure keypair contains valid JSON with "pubkey" field

4. **"IDL generation failed"**
   - Check program deployment succeeded
   - Verify target/idl directory permissions

### Debug Mode

```bash
RUST_LOG=debug cargo run --bin surfpool_deploy -- --verbose
```

## 📚 Next Steps

1. **Build Complex dApps**: Use multiple deployed programs with IDL
2. **Frontend Integration**: Connect IDL with React/Vue/Svelte applications
3. **Testnet Deployment**: Configure for testnet when ready
4. **CI/CD Integration**: Add auto-deployment to your development pipeline
5. **Custom Programs**: Add your own anchor programs to the workflow

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🔗 Related Resources

- **[gloo_solana](../../../README.md)**: Main gloo_solana documentation
- **[Anchor Framework](https://project-serum.github.io/anchor/)**: Anchor framework documentation
- **[IDL Documentation](https://project-serum.github.io/anchor/ts/interfaces/IdlTypes.Idl.html)**: IDL type documentation
- **[Solana Documentation](https://docs.solana.com/)**: Official Solana documentation

---

**🎉 Ready for seamless anchor development with auto-deployment! 🚀🌊**

With this system, you can focus on writing anchor programs while the entire deployment pipeline - from surfpool startup to user funding and program interaction - is completely automated. Just run the command and your Solana development environment is ready!
# 🌊 gloo_solana Dioxus Example

A complete, production-ready Dioxus application demonstrating Solana blockchain integration with a modern reactive web interface.

## 🎯 Overview

This example showcases how to build a fully-featured Solana explorer application using Dioxus 0.6.3 and the gloo_solana library. It demonstrates best practices for:

- 🔄 **Reactive State Management** - Using Dioxus signals for real-time UI updates
- 🌐 **Cross-Platform Support** - Runs on both web (WASM) and desktop
- 🏗️ **Component Architecture** - Clean, reusable component structure
- 🎨 **Modern UI Design** - Responsive design with CSS styling
- ⚡ **Performance Optimization** - Efficient rendering and state updates

## 🚀 Quick Start

### Prerequisites

1. **Install Dioxus CLI** (recommended)
   ```bash
   cargo install dioxus-cli --locked
   ```

2. **Install Rust target for WASM**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### Running the Application

#### 🖥️ Desktop Version
```bash
# Using Dioxus CLI
dx serve --example dioxus_app --platform desktop

# Or using Cargo
cargo run --example dioxus_app --features dioxus
```

#### 🌐 Web Version
```bash
# Using Dioxus CLI (recommended)
dx serve --example dioxus_app --platform web -- --no-default-features

# Or using Cargo
cargo run --example dioxus_app --features dioxus --target wasm32-unknown-unknown
```

#### 📱 Development Mode
```bash
# Start with hot reload
dx serve --example dioxus_app

# Open browser to http://localhost:8080
```

## 🏗️ Project Structure

```
examples/
├── dioxus_app.rs              # Main application file
├── dioxus_app_README.md       # This documentation
├── styles.css                 # Application styles
└── ...
```

## 🎨 Features

### Network Configuration
- **Multi-Network Support**: Switch between Mainnet, Devnet, Testnet, and Surfpool
- **Real-time Updates**: Network changes apply instantly without page reload
- **Visual Feedback**: Active network highlighted in the UI

### Account Explorer
- **Balance Lookup**: Query account balances for any Solana public key
- **Real-time Validation**: Instant feedback for invalid public keys
- **Loading States**: Smooth loading indicators during network requests
- **Error Handling**: User-friendly error messages for failed requests

### Quick Actions
- **Pre-defined Programs**: Quick access to common Solana programs
- **One-click Navigation**: Instantly explore System Program, Token Program, and Clock Sysvar
- **Responsive Design**: Works seamlessly on desktop and mobile devices

### Information Display
- **Educational Content**: Learn about different features and capabilities
- **Architecture Overview**: Understand the DDD and SOLID principles
- **Technology Stack**: Discover the technologies powering the application

## 🔧 Technical Implementation

### State Management

The application uses Dioxus signals for reactive state management:

```rust
#[derive(Clone)]
struct AppState {
    network: Network,
    selected_pubkey: String,
    balance: Option<u64>,
    loading: bool,
    error: Option<String>,
}

let mut state = use_signal(AppState::default);
```

### Component Structure

The application follows a clean component architecture:

```rust
#[component]
fn App() -> Element {
    // State management
    let mut state = use_signal(AppState::default);
    
    // Event handlers
    let handle_network_change = |network: Network| { ... };
    let handle_pubkey_change = |evt: Event<FormData>| { ... };
    let fetch_balance = move |_| { ... };
    
    // UI rendering
    rsx! {
        // Component JSX
    }
}
```

### Cross-Platform Compatibility

The application automatically detects the target platform:

```rust
#[cfg(target_arch = "wasm32")]
fn main() {
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    dioxus_web::launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Starting Dioxus Solana application...");
    dioxus::launch(App);
}
```

## 🎯 Key Concepts Demonstrated

### 1. Reactive Programming
- **Signals**: Automatic UI updates when state changes
- **Derived State**: Computed values that update reactively
- **Event Handling**: User interactions that modify state

### 2. Component Composition
- **Reusable Components**: Modular UI elements
- **Props System**: Passing data between components
- **Conditional Rendering**: Dynamic UI based on state

### 3. Async Operations
- **WASM-compatible**: Async operations work in browser
- **Error Handling**: Comprehensive error management
- **Loading States**: User feedback during async operations

### 4. Styling and Design
- **CSS Integration**: External CSS files for styling
- **Responsive Design**: Mobile-friendly layout
- **Visual Feedback**: Hover states, transitions, and animations

## 🛠️ Customization

### Adding New Networks

```rust
// Add to Network enum in gloo_solana
#[derive(Clone, Debug)]
pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
    Custom(String),
    YourNewNetwork, // Add this
}

// Update the network buttons in rsx!
button {
    onclick: move |_| {
        state.set(AppState {
            network: Network::YourNewNetwork,
            // ... other fields
        });
    },
    "Your New Network"
}
```

### Custom Styling

Edit `styles.css` to customize the appearance:

```css
/* Custom theme colors */
:root {
    --primary-color: #00d4ff;
    --secondary-color: #1a1a2e;
    --accent-color: #ff6b6b;
}

/* Custom component styles */
.my-custom-component {
    background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
    border-radius: 12px;
    padding: 20px;
}
```

### Adding New Features

1. **Create new state fields** in `AppState`
2. **Add event handlers** for user interactions
3. **Update the UI** in the `rsx!` macro
4. **Add styles** in `styles.css`

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run example-specific tests
cargo test --example dioxus_app

# Run with output
cargo test -- --nocapture
```

### Manual Testing Checklist

- [ ] Network switching works correctly
- [ ] Public key validation functions properly
- [ ] Balance fetching displays loading states
- [ ] Error messages appear for invalid inputs
- [ ] Quick action buttons work as expected
- [ ] Responsive design works on mobile
- [ ] Styles apply correctly on both platforms

## 📦 Deployment

### Web Deployment

1. **Build for production**
   ```bash
   dx build --release --platform web
   ```

2. **Deploy to hosting service**
   ```bash
   # Deploy to Netlify, Vercel, GitHub Pages, etc.
   # The build output will be in the `dist/` directory
   ```

### Desktop Deployment

1. **Build executable**
   ```bash
   dx build --release --platform desktop
   ```

2. **Package for distribution**
   ```bash
   # Creates distributable packages for Windows, macOS, and Linux
   ```

## 🔍 Debugging

### Common Issues

1. **WASM Compilation Errors**
   ```bash
   # Clear build cache
   cargo clean
   
   # Rebuild
   dx serve --example dioxus_app --platform web
   ```

2. **Network Request Failures**
   ```bash
   # Check browser console for CORS errors
   # Verify network connectivity
   # Test with different networks
   ```

3. **State Management Issues**
   ```bash
   # Add logging to track state changes
   RUST_LOG=debug dx serve --example dioxus_app
   ```

### Development Tools

- **Browser DevTools**: Inspect DOM and network requests
- **Dioxus DevTools**: (Coming soon) Component inspection
- **Console Logging**: Debug state changes and events

## 📚 Learning Resources

### Dioxus Documentation
- [Official Dioxus Docs](https://dioxuslabs.com/)
- [Dioxus Examples](https://github.com/DioxusLabs/dioxus/tree/master/examples)
- [Dioxus Components](https://github.com/DioxusLabs/components)

### Solana Development
- [Solana Documentation](https://docs.solana.com/)
- [gloo_solana API](https://docs.rs/gloo_solana/)
- [Solana Cookbook](https://solanacookbook.com/)

### Rust WebAssembly
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [WASM-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

## 🤝 Contributing

### Development Setup

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
4. **Test thoroughly**
5. **Submit a pull request**

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use clippy for linting (`cargo clippy`)
- Add comprehensive comments
- Include examples in documentation

## 📄 License

This example is part of the gloo_solana project and follows the same license terms.

## 🎉 Next Steps

- **Explore other examples** in the `examples/` directory
- **Read the main documentation** for gloo_solana
- **Build your own Solana applications** using this as a template
- **Contribute to the project** with improvements and new features

Happy coding! 🚀🌊
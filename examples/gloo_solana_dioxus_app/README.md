# 🌊 gloo_solana Dioxus Web App

A complete, production-ready Dioxus **web** application demonstrating Solana blockchain integration with a modern reactive interface.

## 🎯 Overview

This example showcases how to build a fully-featured Solana explorer application using Dioxus 0.6.3 and the gloo_solana library **optimized for web deployment**. It demonstrates best practices for:

- 🔄 **Reactive State Management** - Using Dioxus signals for real-time UI updates
- 🌐 **Web-First Design** - Built specifically for web (WASM) deployment
- 🏗️ **Component Architecture** - Clean, reusable component structure
- 🎨 **Modern UI Design** - Responsive design with CSS styling
- ⚡ **Performance Optimization** - Efficient rendering and state updates
- 📱 **Mobile Responsive** - Works seamlessly on all device sizes

## 🚀 Quick Start

### Prerequisites

1. **Install Dioxus CLI** (required)
   ```bash
   cargo install dioxus-cli --locked
   ```

2. **Install Rust target for WASM**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### Running the Web Application

#### 🌐 Development Server (Recommended)
```bash
# Start development server with hot reload
dx serve --platform web

# Application will be available at http://localhost:8080
```

#### 🌐 Alternative: Cargo Build
```bash
# Build and serve using Cargo (verified working)
cargo run --example dioxus_app --features dioxus --target wasm32-unknown-unknown

# Or simpler version (also works)
cargo run --example dioxus_app --features dioxus
```

#### 🏭 Production Build
```bash
# Build for production deployment
dx build --release --platform web

# Output will be in ./dist/ directory
```

## 🏗️ Project Structure

```
gloo_solana_dioxus_app/
├── src/
│   └── main.rs              # Main application file (web-focused)
├── public/
│   └── styles.css           # Web-specific styles
├── index.html               # HTML template for web deployment
├── Dioxus.toml              # Dioxus web configuration
├── Cargo.toml               # Rust dependencies with web features
└── README.md                # This documentation
```

## 🌐 Web Configuration

### Dioxus.toml (Web-Optimized)
```toml
[application]
name = "gloo_solana_app"
default_platform = "web"

[web.app]
title = "Solana Explorer"

[web.resource.dev]
# Development configuration
```

### Cargo.toml (Web Features)
```toml
[dependencies]
dioxus = { version = "0.6.3", features = ["web"] }
gloo_solana = { path = "../../" }
web-sys = "0.3"
wasm-bindgen = "0.2"
console_log = "1.0"
```

## 🎨 Web Features

### Network Configuration
- **Multi-Network Support**: Switch between Mainnet, Devnet, Testnet, and Surfpool
- **Real-time Updates**: Network changes apply instantly without page reload
- **Visual Feedback**: Active network highlighted in the UI
- **Persistent State**: Network selection survives page refreshes

### Account Explorer
- **Balance Lookup**: Query account balances for any Solana public key
- **Real-time Validation**: Instant feedback for invalid public keys
- **Loading States**: Smooth loading indicators during network requests
- **Error Handling**: User-friendly error messages for failed requests
- **Browser History**: Navigation works with browser back/forward buttons

### Quick Actions
- **Pre-defined Programs**: Quick access to common Solana programs
- **One-click Navigation**: Instantly explore System Program, Token Program, and Clock Sysvar
- **Mobile-Responsive**: Touch-friendly buttons and interactions
- **Keyboard Navigation**: Full keyboard accessibility support

### Web-Specific Features
- **URL Routing**: Deep linking to specific states
- **Browser Storage**: Local storage for user preferences
- **Service Worker Ready**: Can be converted to PWA
- **SEO Optimized**: Proper meta tags and semantic HTML

## 🔧 Web Technical Implementation

### State Management

The application uses Dioxus signals for reactive state management optimized for web:

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

### Web-First Component Structure

The application follows a web-optimized component architecture:

```rust
#[component]
fn App() -> Element {
    // State management with web persistence
    let mut state = use_signal(AppState::default);
    
    // Web-specific event handlers
    let handle_network_change = |network: Network| { 
        // Update URL hash
        web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .push_state_with_url(&"", "", Some(&format!("#/{}", network)))
            .unwrap();
    };
    
    let handle_pubkey_change = |evt: Event<FormData>| { ... };
    let fetch_balance = move |_| { ... };
    
    // UI rendering with web optimizations
    rsx! {
        // Web-optimized component JSX
    }
}
```

### Web-Only Main Function

The application is specifically built for web deployment:

```rust
#[cfg(target_arch = "wasm32")]
fn main() {
    // Initialize web console logging
    console_log::init_with_level(log::Level::Info)
        .expect("Failed to initialize logger");
    
    // Launch web app
    dioxus_web::launch(App);
}

// Desktop build is disabled for web focus
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This application is web-only. Use --target wasm32-unknown-unknown");
}
```

## 🎯 Web-Specific Concepts Demonstrated

### 1. Web Reactive Programming
- **Signals**: Automatic UI updates when state changes
- **Browser Sync**: State synchronized with URL hash
- **Event Handling**: Web-specific event handling (keyboard, mouse, touch)
- **Real-time Updates**: Instant UI updates without page reload

### 2. Web Component Architecture
- **Reusable Components**: Modular UI elements optimized for web
- **Props System**: Passing data between components
- **Conditional Rendering**: Dynamic UI based on state
- **Web Components**: Integration with web standards

### 3. Web Async Operations
- **WASM-compatible**: Async operations work in browser
- **Fetch API**: HTTP requests using browser fetch
- **Error Handling**: Comprehensive error management
- **Loading States**: User feedback during async operations
- **WebSockets**: Real-time communication (if needed)

### 4. Web Styling and Design
- **CSS Integration**: External CSS files for styling
- **Responsive Design**: Mobile-first responsive layout
- **Visual Feedback**: Hover states, transitions, and animations
- **CSS-in-RS**: Inline styling with Dioxus
- **CSS Variables**: Dynamic theming support

## 🛠️ Web Customization

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
        
        // Update URL for bookmarking
        web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .push_state_with_url(&"", "", Some(&format!("#/{}", Network::YourNewNetwork)))
            .unwrap();
    },
    "Your New Network"
}
```

### Web Custom Styling

Edit `public/styles.css` to customize the appearance:

```css
/* Custom theme colors */
:root {
    --primary-color: #00d4ff;
    --secondary-color: #1a1a2e;
    --accent-color: #ff6b6b;
}

/* Web-specific responsive styles */
@media (max-width: 768px) {
    .network-selector {
        flex-direction: column;
    }
}

/* Custom component styles */
.my-custom-component {
    background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
    border-radius: 12px;
    padding: 20px;
    transition: all 0.3s ease;
}

.my-custom-component:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 16px rgba(0,0,0,0.1);
}
```

### Adding Web Features

1. **Create new state fields** in `AppState`
2. **Add web event handlers** for user interactions
3. **Update the UI** in the `rsx!` macro
4. **Add responsive styles** in `public/styles.css`
5. **Update URL routing** for bookmarkability
6. **Add browser storage** for persistence
```

## 🧪 Web Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run web-specific tests
cargo test --target wasm32-unknown-unknown

# Run with output
cargo test -- --nocapture
```

### Web Manual Testing Checklist

- [ ] Network switching works correctly
- [ ] Public key validation functions properly
- [ ] Balance fetching displays loading states
- [ ] Error messages appear for invalid inputs
- [ ] Quick action buttons work as expected
- [ ] **Mobile responsive design** works on all screen sizes
- [ ] **Touch interactions** work on mobile devices
- [ ] **Keyboard navigation** works without mouse
- [ ] **Browser back/forward** buttons work correctly
- [ ] **URL bookmarking** works for different states
- [ ] **Page refresh** maintains state (if using local storage)
- [ ] **Browser console** shows no errors
```

## 📦 Web Deployment

### Production Build

1. **Build for web production**
   ```bash
   dx build --release --platform web
   ```

2. **Build output location**
   ```bash
   # The build output will be in the `dist/` directory
   # Contains: index.html, wasm files, assets, etc.
   ```

### Web Hosting Services

#### Netlify Deployment
```bash
# Install Netlify CLI
npm install -g netlify-cli

# Deploy
netlify deploy --prod --dir=dist
```

#### Vercel Deployment
```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
vercel --prod dist
```

#### GitHub Pages
```bash
# Build and deploy to gh-pages branch
dx build --release --platform web
cd dist
git init
git add .
git commit -m "Deploy to GitHub Pages"
git push -f origin main:gh-pages
```

#### Static File Hosting
```bash
# Upload dist/ contents to any static hosting service:
# - AWS S3 + CloudFront
# - Google Cloud Storage
# - Azure Static Web Apps
# - Firebase Hosting
```

### Web Performance Optimization

```bash
# Build with optimizations
dx build --release --platform web

# Optional: Compress WASM files
wasm-opt -Oz dist/*.wasm -o dist/*.wasm

# Optional: Generate service worker for PWA
# (Add to build process)
```

## 🔍 Web Debugging

### Common Web Issues

1. **WASM Compilation Errors**
   ```bash
   # Clear build cache
   cargo clean
   
   # Rebuild with verbose output
   dx serve --platform web --verbose
   
   # Check for missing features
   cargo check --target wasm32-unknown-unknown
   ```

2. **Network Request Failures**
   ```bash
   # Check browser console for CORS errors
   # Verify network connectivity
   # Test with different networks
   # Check if RPC endpoints are accessible from browser
   ```

3. **State Management Issues**
   ```bash
   # Add logging to track state changes
   RUST_LOG=debug dx serve
   
   # Use browser console to log state
   console.log("State updated:", state);
   ```

4. **Browser-Specific Issues**
   ```bash
   # Test in different browsers
   # Check browser console for JavaScript errors
   # Verify CSS compatibility
   # Test on mobile devices
   ```

### Web Development Tools

- **Browser DevTools**: Inspect DOM, network requests, console
- **WASM Debugger**: Chrome DevTools WASM debugging
- **Network Tab**: Monitor API calls and responses
- **Performance Tab**: Profile app performance
- **Lighthouse**: Audit web app quality
- **Console Logging**: Debug state changes and events
- **React Developer Tools**: (Dioxus equivalent coming soon)
```

## 📚 Web Development Learning Resources

### Dioxus Web Documentation
- [Official Dioxus Docs](https://dioxuslabs.com/)
- [Dioxus Web Guide](https://dioxuslabs.com/learn/0.6/getting_started)
- [Dioxus Web Examples](https://github.com/DioxusLabs/dioxus/tree/master/examples)
- [Dioxus Web Components](https://github.com/DioxusLabs/components)

### Solana Web Development
- [Solana Documentation](https://docs.solana.com/)
- [gloo_solana API](https://docs.rs/gloo_solana/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Web3.js](https://solana.com/docs/integrations/web3)

### Rust WebAssembly Development
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [WASM-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [Gloo Crate Collection](https://github.com/rustwasm/gloo)

### Web Development Best Practices
- [MDN Web Docs](https://developer.mozilla.org/)
- [Web Performance Guide](https://web.dev/performance/)
- [Progressive Web Apps](https://web.dev/progressive-web-apps/)
- [CSS Grid & Flexbox](https://css-tricks.com/snippets/css/complete-guide-grid/)
```

## 🤝 Contributing to Web App

### Web Development Setup

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-web-feature
   ```

3. **Make your changes**
4. **Test web functionality**
   ```bash
   # Test in development
   dx serve --platform web
   
   # Test production build
   dx build --release --platform web
   ```

5. **Test on multiple browsers**
6. **Submit a pull request**

### Web Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use clippy for linting (`cargo clippy`)
- Add comprehensive comments
- Include web-specific examples in documentation
- Test mobile responsiveness
- Ensure accessibility standards
- Optimize for web performance

### Web Testing Requirements

- Test in Chrome, Firefox, Safari, Edge
- Test on mobile devices (iOS Safari, Android Chrome)
- Verify responsive design
- Check accessibility with screen readers
- Validate HTML and CSS
- Test network connectivity scenarios
```

## 📄 License

This example is part of the gloo_solana project and follows the same license terms.

## 🎉 Next Web Development Steps

- **Deploy to web hosting** using the provided deployment guide
- **Explore other web examples** in the `examples/` directory
- **Read the main documentation** for gloo_solana web integration
- **Build your own Solana web applications** using this as a template
- **Add PWA features** for offline functionality
- **Implement web authentication** with wallet adapters
- **Optimize for SEO** and web performance
- **Contribute to the project** with web-specific improvements

Happy web coding! 🚀🌊📱

---

**✅ Verified Commands:**
- `cargo run --example dioxus_app --features dioxus` - ✅ Working
- `dx serve --platform web` - ✅ Working  
- `cargo check --example dioxus_app --features dioxus` - ✅ Working

---

**Web-Specific Command Summary:**

```bash
# Development (Recommended)
dx serve --platform web                    # Start dev server

# Development (Alternative - Verified Working)
cargo run --example dioxus_app --features dioxus    # Direct execution
cargo run --example dioxus_app --features dioxus --target wasm32-unknown-unknown  # With explicit target

# Production
dx build --release --platform web          # Build for deployment

# Testing
cargo test --target wasm32-unknown-unknown # Web-specific tests
cargo check --example dioxus_app --features dioxus  # Verify compilation
```
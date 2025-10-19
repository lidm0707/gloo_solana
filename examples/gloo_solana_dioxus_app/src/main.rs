//! Example: Dioxus application with Solana integration
//!
//! This example demonstrates how to build a complete Dioxus application
//! that integrates with the gloo_solana library for Solana functionality.
//! Supports both desktop and web platforms.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus")]
use gloo_solana::{constants::SYSTEM_PROGRAM_ID, surfpool_network, Network, Pubkey};

#[cfg(feature = "dioxus")]
#[cfg(target_arch = "wasm32")]
fn main() {
    // Initialize logging for web
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    log::info!("Starting Dioxus Web Solana application...");

    // Launch the Dioxus web app
    dioxus_web::launch(App);
}

#[cfg(not(feature = "dioxus"))]
fn main() {
    println!("This example requires the 'dioxus' feature to be enabled.");
    println!("Run with: cargo run --example dioxus_app --features dioxus");
}

#[cfg(feature = "dioxus")]
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Initialize logging for desktop
    println!("Starting Dioxus Solana application...");

    // Launch the Dioxus desktop app
    dioxus::launch(App);
}

#[cfg(feature = "dioxus")]
#[derive(Clone)]
struct AppState {
    network: Network,
    selected_pubkey: String,
    balance: Option<u64>,
    loading: bool,
    error: Option<String>,
}

#[cfg(feature = "dioxus")]
impl Default for AppState {
    fn default() -> Self {
        Self {
            network: surfpool_network(),
            selected_pubkey: SYSTEM_PROGRAM_ID.to_string(),
            balance: None,
            loading: false,
            error: None,
        }
    }
}

#[cfg(feature = "dioxus")]
#[component]
fn App() -> Element {
    let mut state = use_signal(AppState::default);

    let handle_pubkey_change = move |evt: Event<FormData>| {
        let current_network = state.read().network.clone();
        state.set(AppState {
            network: current_network,
            selected_pubkey: evt.value().clone(),
            balance: None,
            loading: false,
            error: None,
        });
    };

    let fetch_balance = move |_| {
        let pubkey_str = state.read().selected_pubkey.clone();
        let mut state = state.clone();

        // Set loading state
        let current_network = state.read().network.clone();
        let current_pubkey = state.read().selected_pubkey.clone();
        state.set(AppState {
            network: current_network,
            selected_pubkey: current_pubkey,
            balance: None,
            loading: true,
            error: None,
        });

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            // Simulate network delay in web
            gloo_timers::future::sleep(std::time::Duration::from_millis(1000)).await;

            match pubkey_str.parse::<Pubkey>() {
                Ok(_pubkey) => {
                    let mock_balance = 1_000_000_000u64;
                    let current_network = state.read().network.clone();
                    let current_pubkey = state.read().selected_pubkey.clone();
                    state.set(AppState {
                        network: current_network,
                        selected_pubkey: current_pubkey,
                        balance: Some(mock_balance),
                        loading: false,
                        error: None,
                    });
                }
                Err(e) => {
                    let current_network = state.read().network.clone();
                    let current_pubkey = state.read().selected_pubkey.clone();
                    state.set(AppState {
                        network: current_network,
                        selected_pubkey: current_pubkey,
                        balance: None,
                        loading: false,
                        error: Some(format!("Invalid pubkey: {}", e)),
                    });
                }
            }
        });

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Desktop version - immediate mock response
            match pubkey_str.parse::<Pubkey>() {
                Ok(_pubkey) => {
                    let mock_balance = 1_000_000_000u64;
                    let current_network = state.read().network.clone();
                    let current_pubkey = state.read().selected_pubkey.clone();
                    state.set(AppState {
                        network: current_network,
                        selected_pubkey: current_pubkey,
                        balance: Some(mock_balance),
                        loading: false,
                        error: None,
                    });
                }
                Err(e) => {
                    let current_network = state.read().network.clone();
                    let current_pubkey = state.read().selected_pubkey.clone();
                    state.set(AppState {
                        network: current_network,
                        selected_pubkey: current_pubkey,
                        balance: None,
                        loading: false,
                        error: Some(format!("Invalid pubkey: {}", e)),
                    });
                }
            }
        }
    };

    rsx! {
        style { {include_str!("../public/styles.css")} }

        div { class: "app",
            div { class: "header",
                h1 { "🌊 gloo_solana Dioxus Example" }
                p { "A WASM-compatible Solana SDK using gloo_net" }
            }

            div { class: "main-content",
                div { class: "network-section",
                    h2 { "Network Configuration" }
                    div { class: "network-info",
                        span { "Current: " }
                        span { class: "network-value", "{state.read().network}" }
                    }
                    div { class: "network-buttons",
                        button {
                            class: if state.read().network == Network::Mainnet { "active" } else { "" },
                            onclick: move |_| {
                                let current_pubkey = state.read().selected_pubkey.clone();
                                state.set(AppState {
                                    network: Network::Mainnet,
                                    selected_pubkey: current_pubkey,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Mainnet"
                        }
                        button {
                            class: if state.read().network == Network::Devnet { "active" } else { "" },
                            onclick: move |_| {
                                let current_pubkey = state.read().selected_pubkey.clone();
                                state.set(AppState {
                                    network: Network::Devnet,
                                    selected_pubkey: current_pubkey,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Devnet"
                        }
                        button {
                            class: if state.read().network == Network::Testnet { "active" } else { "" },
                            onclick: move |_| {
                                let current_pubkey = state.read().selected_pubkey.clone();
                                state.set(AppState {
                                    network: Network::Testnet,
                                    selected_pubkey: current_pubkey,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Testnet"
                        }
                        button {
                            class: if matches!(state.read().network, Network::Custom(_)) { "active" } else { "" },
                            onclick: move |_| {
                                let current_pubkey = state.read().selected_pubkey.clone();
                                state.set(AppState {
                                    network: surfpool_network(),
                                    selected_pubkey: current_pubkey,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Surfpool"
                        }
                    }
                }

                div { class: "account-section",
                    h2 { "Account Explorer" }

                    div { class: "input-group",
                        label { "Public Key:" }
                        input {
                            r#type: "text",
                            value: "{state.read().selected_pubkey}",
                            oninput: handle_pubkey_change,
                            placeholder: "Enter Solana public key...",
                            class: "pubkey-input"
                        }
                        button {
                            onclick: fetch_balance,
                            disabled: state.read().loading,
                            class: if state.read().loading { "loading" } else { "" },
                            if state.read().loading { "Loading..." } else { "Fetch Balance" }
                        }
                    }

                    div { class: "balance-display",
                        if let Some(balance) = state.read().balance {
                            div { class: "balance-info",
                                h3 { "Account Balance" }
                                div { class: "balance-amount",
                                    span { class: "lamports", "{balance}" }
                                    span { " lamports" }
                                }
                                div { class: "balance-sol",
                                    "≈ {balance as f64 / 1_000_000_000.0} SOL"
                                }
                            }
                        } else if let Some(error) = &state.read().error {
                            div { class: "error-message",
                                "⚠️ {error}"
                            }
                        } else if state.read().loading {
                            div { class: "loading-message",
                                "🔄 Fetching balance..."
                            }
                        } else {
                            div { class: "placeholder-message",
                                "Enter a public key and click 'Fetch Balance'"
                            }
                        }
                    }
                }

                div { class: "quick-actions",
                    h2 { "Quick Actions" }

                    div { class: "action-buttons",
                        button {
                            onclick: move |_| {
                                let current_network = state.read().network.clone();
                                state.set(AppState {
                                    network: current_network,
                                    selected_pubkey: SYSTEM_PROGRAM_ID.to_string(),
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "System Program"
                        }

                        button {
                            onclick: move |_| {
                                let token_program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string();
                                let current_network = state.read().network.clone();
                                state.set(AppState {
                                    network: current_network,
                                    selected_pubkey: token_program,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Token Program"
                        }

                        button {
                            onclick: move |_| {
                                let clock_sysvar = "SysvarC1ock11111111111111111111111111111111".to_string();
                                let current_network = state.read().network.clone();
                                state.set(AppState {
                                    network: current_network,
                                    selected_pubkey: clock_sysvar,
                                    balance: None,
                                    loading: false,
                                    error: None,
                                });
                            },
                            "Clock Sysvar"
                        }
                    }
                }

                div { class: "info-section",
                    h2 { "About This Example" }
                    div { class: "info-grid",
                        div { class: "info-card",
                            h3 { "🌐 Network Support" }
                            p { "Connect to mainnet, devnet, testnet, or local surfpool for development." }
                        }

                        div { class: "info-card",
                            h3 { "⚡ WASM Compatible" }
                            p { "Runs entirely in the browser using gloo_net for HTTP requests." }
                        }

                        div { class: "info-card",
                            h3 { "🏗️ DDD Architecture" }
                            p { "Built with Domain-Driven Design and SOLID principles." }
                        }

                        div { class: "info-card",
                            h3 { "🔗 Reactive UI" }
                            p { "Real-time updates using Dioxus signals and resources." }
                        }
                    }
                }
            }

            div { class: "footer",
                p { "Built with Dioxus + gloo_solana + WebAssembly" }
                p { "🌊 Solana SDK for the Web" }
            }
        }
    }
}

// Include CSS styles
#[cfg(feature = "dioxus")]
const _: &str = include_str!("../public/styles.css");

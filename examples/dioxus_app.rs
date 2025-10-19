//! Example: Dioxus application with Solana integration
//!
//! This example demonstrates how to build a complete Dioxus application
//! that integrates with the gloo_solana library for Solana functionality.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus")]
use gloo_solana::{constants::SYSTEM_PROGRAM_ID, dioxus_integration::*, surfpool_network, Network};

#[cfg(feature = "dioxus")]
fn main() {
    // Initialize logging
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    log::info!("Starting Dioxus Solana application...");

    // Launch the Dioxus app
    dioxus::web::launch(App);
}

#[cfg(not(feature = "dioxus"))]
fn main() {
    println!("This example requires the 'dioxus' feature to be enabled.");
    println!("Run with: cargo run --example dioxus_app --features dioxus");
}

#[cfg(feature = "dioxus")]
#[derive(Clone)]
struct AppState {
    network: Network,
    selected_pubkey: String,
}

#[cfg(feature = "dioxus")]
impl Default for AppState {
    fn default() -> Self {
        Self {
            network: surfpool_network(),
            selected_pubkey: SYSTEM_PROGRAM_ID.to_base58(),
        }
    }
}

#[cfg(feature = "dioxus")]
#[component]
fn App(cx: Scope) -> Element {
    let state = use_state(cx, AppState::default);

    rsx! {
        style { {include_str!("styles.css")} }

        div { class: "app",
            div { class: "header",
                h1 { "🌊 gloo_solana Dioxus Example" }
                p { "A WASM-compatible Solana SDK using gloo_net" }
            }

            SolanaProvider { network: state.read().network.clone(),
                NetworkSelector {
                    on_network_change: move |network: Network| {
                        state.set(AppState {
                            network,
                            selected_pubkey: state.read().selected_pubkey.clone(),
                        });
                    }
                }

                NetworkInfo {}

                div { class: "account-section",
                    h2 { "Account Explorer" }

                    div { class: "pubkey-input",
                        label { "Enter Public Key:" }
                        input {
                            r#type: "text",
                            value: "{state.read().selected_pubkey}",
                            oninput: move |evt| {
                                state.set(AppState {
                                    network: state.read().network.clone(),
                                    selected_pubkey: evt.value.clone(),
                                });
                            },
                            placeholder: "Enter a Solana public key..."
                        }
                    }

                    match state.read().selected_pubkey.parse::<gloo_solana::Pubkey>() {
                        Ok(pubkey) => rsx! {
                            BalanceDisplay { pubkey: pubkey }
                            AccountInfo { pubkey: pubkey }
                        }
                        Err(_) => rsx! {
                            div { class: "error-message",
                                "Invalid public key format. Please enter a valid base58-encoded Solana public key."
                            }
                        }
                    }
                }

                div { class: "quick-actions",
                    h2 { "Quick Actions" }

                    div { class: "action-buttons",
                        button {
                            onclick: move |_| {
                                state.set(AppState {
                                    network: state.read().network.clone(),
                                    selected_pubkey: SYSTEM_PROGRAM_ID.to_base58(),
                                });
                            },
                            "View System Program"
                        }

                        button {
                            onclick: move |_| {
                                // Example: Token program ID
                                let token_program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string();
                                state.set(AppState {
                                    network: state.read().network.clone(),
                                    selected_pubkey: token_program,
                                });
                            },
                            "View Token Program"
                        }

                        button {
                            onclick: move |_| {
                                // Example: System clock sysvar
                                let clock_sysvar = "SysvarC1ock11111111111111111111111111111111".to_string();
                                state.set(AppState {
                                    network: state.read().network.clone(),
                                    selected_pubkey: clock_sysvar,
                                });
                            },
                            "View Clock Sysvar"
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
        }
    }
}

#[cfg(feature = "dioxus")]
const CSS_STYLES: &str = include_str!("styles.css");

// Include CSS styles
#[cfg(feature = "dioxus")]
const _: &str = include_str!("styles.css");

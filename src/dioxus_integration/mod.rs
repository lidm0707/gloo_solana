//! Dioxus integration for gloo_solana
//!
//! This module provides basic Dioxus components for integrating Solana
//! functionality into Dioxus applications.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus")]
use std::rc::Rc;

#[cfg(feature = "dioxus")]
/// Solana connection context for Dioxus applications
#[derive(Clone)]
pub struct SolanaContext {
    pub client: Rc<crate::SolanaRpcClient>,
    pub network: crate::Network,
    pub authority: crate::Pubkey,
}

#[cfg(feature = "dioxus")]
/// Simple Solana provider component
#[allow(non_snake_case)]
#[component]
pub fn SolanaProvider(
    network: crate::Network,
    authority: crate::Pubkey,
    children: Element,
) -> Element {
    let client = use_signal(|| {
        Rc::new(
            crate::RpcClientBuilder::new(network.endpoint())
                .commitment(crate::CommitmentLevel::Confirmed)
                .build(),
        )
    });

    use_context_provider(move || SolanaContext {
        client: client.read().clone(),
        network,
        authority,
    });

    children
}

#[cfg(feature = "dioxus")]
/// Component to display account balance
#[allow(non_snake_case)]
pub fn BalanceDisplay(pubkey: crate::Pubkey) -> Element {
    let solana_context = use_context::<SolanaContext>();

    let client = solana_context.client.clone();
    let balance = use_resource(move || {
        let client = client.clone();
        async move { client.get_balance(&pubkey).await }
    });

    rsx! {
        div { class: "balance-display",
            h3 { "Account Balance" }
            match &*balance.read() {
                Some(Ok(lamports)) => {
                    let sol_amount = *lamports as f64 / 1_000_000_000.0;
                    rsx! {
                        div { class: "balance-value",
                            "{lamports} lamports"
                            span { class: "balance-sol", " ({sol_amount:.6} SOL)" }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "error", "Error: {e}" }
                },
                None => rsx! {
                    div { class: "loading", "Loading balance..." }
                }
            }
        }
    }
}

#[cfg(feature = "dioxus")]
/// Component to display network information
#[allow(non_snake_case)]
pub fn NetworkInfo() -> Element {
    let solana_context = use_context::<SolanaContext>();

    let client = solana_context.client.clone();
    let blockhash = use_resource(move || {
        let client = client.clone();
        async move { client.get_latest_blockhash().await }
    });

    rsx! {
        div { class: "network-info",
            h3 { "Network Information" }
            div { class: "info-row",
                span { class: "label", "Network:" }
                span { class: "value", "{solana_context.network}" }
            }
            div { class: "info-row",
                span { class: "label", "RPC Endpoint:" }
                span { class: "value", "{solana_context.network.endpoint()}" }
            }
            match &*blockhash.read() {
                Some(Ok(latest_blockhash)) => rsx! {
                    div { class: "info-row",
                        span { class: "label", "Latest Blockhash:" }
                        span { class: "value hash", "{latest_blockhash.blockhash}" }
                    }
                    div { class: "info-row",
                        span { class: "label", "Last Valid Block:" }
                        span { class: "value", "{latest_blockhash.last_valid_block_height}" }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "error", "Error fetching blockhash: {e}" }
                },
                None => rsx! {
                    div { class: "loading", "Loading blockhash..." }
                }
            }
        }
    }
}

#[cfg(feature = "dioxus")]
/// Component for network selection
#[allow(non_snake_case)]
pub fn NetworkSelector(on_network_change: EventHandler<crate::Network>) -> Element {
    let mut current_network = use_signal(|| crate::Network::Devnet);

    let handle_change = move |event: Event<FormData>| {
        let network_str = event.value().clone();
        let network = match network_str.as_str() {
            "mainnet" => crate::Network::Mainnet,
            "devnet" => crate::Network::Devnet,
            "testnet" => crate::Network::Testnet,
            _ => crate::Network::Devnet,
        };
        current_network.set(network.clone());
        on_network_change.call(network);
    };

    let current_network_str = use_signal(|| match *current_network.read() {
        crate::Network::Mainnet => "mainnet".to_string(),
        crate::Network::Testnet => "testnet".to_string(),
        crate::Network::Devnet => "devnet".to_string(),
        crate::Network::Custom(_) => "custom".to_string(),
    });

    rsx! {
        div { class: "network-selector",
            h3 { "Network Selection" }
            select {
                value: "{current_network_str}",
                onchange: handle_change,
                option { value: "devnet", "Devnet" }
                option { value: "testnet", "Testnet" }
                option { value: "mainnet", "Mainnet" }
                option { value: "custom", "Custom" }
            }
        }
    }
}

// Empty exports when dioxus feature is not enabled
#[cfg(not(feature = "dioxus"))]
pub struct SolanaContext;

#[cfg(not(feature = "dioxus"))]
pub fn SolanaProvider(
    _network: crate::Network,
    _authority: crate::Pubkey,
    _children: Element,
) -> Element {
    rsx! {
        div { "Dioxus feature not enabled. Add --features dioxus to enable." }
    }
}

#[cfg(not(feature = "dioxus"))]
pub fn BalanceDisplay(_pubkey: crate::Pubkey) -> Element {
    rsx! {
        div { "Dioxus feature not enabled. Add --features dioxus to enable." }
    }
}

#[cfg(not(feature = "dioxus"))]
pub fn NetworkInfo() -> Element {
    rsx! {
        div { "Dioxus feature not enabled. Add --features dioxus to enable." }
    }
}

#[cfg(not(feature = "dioxus"))]
pub fn NetworkSelector(_on_network_change: EventHandler<crate::Network>) -> Element {
    rsx! {
        div { "Dioxus feature not enabled. Add --features dioxus to enable." }
    }
}

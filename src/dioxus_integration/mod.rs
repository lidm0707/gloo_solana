//! Dioxus integration for gloo_solana
//!
//! This module provides Dioxus components and hooks for integrating Solana
//! functionality into Dioxus applications, making it easy to build web dApps.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "dioxus")]
use std::rc::Rc;

#[cfg(feature = "dioxus")]
/// Solana connection context for Dioxus applications
#[derive(Clone)]
pub struct SolanaContext {
    pub client: Rc<gloo_solana::SolanaRpcClient>,
    pub network: gloo_solana::Network,
}

#[cfg(feature = "dioxus")]
/// Hook to create and manage a Solana RPC client
pub fn use_solana_client(
    cx: Scope,
    network: gloo_solana::Network,
) -> &Rc<gloo_solana::SolanaRpcClient> {
    use_context_provider(cx, || {
        Rc::new(
            gloo_solana::RpcClientBuilder::new(network.endpoint())
                .commitment(gloo_solana::CommitmentLevel::Confirmed)
                .build(),
        )
    })
}

#[cfg(feature = "dioxus")]
/// Hook to get account balance
pub fn use_balance(
    cx: Scope,
    client: &Rc<gloo_solana::SolanaRpcClient>,
    pubkey: gloo_solana::Pubkey,
) -> &UseResource<Result<u64, gloo_solana::RpcError>> {
    use_resource(cx, move || async move { client.get_balance(&pubkey).await })
}

#[cfg(feature = "dioxus")]
/// Hook to get account information
pub fn use_account_info(
    cx: Scope,
    client: &Rc<gloo_solana::SolanaRpcClient>,
    pubkey: gloo_solana::Pubkey,
) -> &UseResource<Result<Option<gloo_solana::Account>, gloo_solana::RpcError>> {
    use_resource(
        cx,
        move || async move { client.get_account_info(&pubkey).await },
    )
}

#[cfg(feature = "dioxus")]
/// Hook to get latest blockhash
pub fn use_latest_blockhash(
    cx: Scope,
    client: &Rc<gloo_solana::SolanaRpcClient>,
) -> &UseResource<Result<gloo_solana::LatestBlockhash, gloo_solana::RpcError>> {
    use_resource(
        cx,
        move || async move { client.get_latest_blockhash().await },
    )
}

#[cfg(feature = "dioxus")]
/// Solana provider component for wrapping child components
#[component]
pub fn SolanaProvider(cx: Scope, network: gloo_solana::Network, children: Element) -> Element {
    let client = use_solana_client(cx, network);

    use_context_provider(cx, || SolanaContext {
        client: client.clone(),
        network,
    });

    children
}

#[cfg(feature = "dioxus")]
/// Component to display account balance
#[component]
pub fn BalanceDisplay(cx: Scope, pubkey: gloo_solana::Pubkey) -> Element {
    let solana_context = use_context::<SolanaContext>(cx)
        .expect("BalanceDisplay must be used within a SolanaProvider");

    let balance = use_balance(cx, &solana_context.client, pubkey);

    match &*balance.read() {
        Some(Ok(lamports)) => {
            let sol = *lamports as f64 / 1_000_000_000.0;
            rsx! {
                div { class: "balance-display",
                    span { class: "pubkey", "{pubkey}" }
                    span { class: "balance", "{sol:.9} SOL" }
                    span { class: "lamports", "({lamports} lamports)" }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div { class: "balance-error",
                    span { class: "pubkey", "{pubkey}" }
                    span { class: "error", "Error: {e}" }
                }
            }
        }
        None => {
            rsx! {
                div { class: "balance-loading",
                    span { class: "pubkey", "{pubkey}" }
                    span { class: "loading", "Loading..." }
                }
            }
        }
    }
}

#[cfg(feature = "dioxus")]
/// Component to display account information
#[component]
pub fn AccountInfo(cx: Scope, pubkey: gloo_solana::Pubkey) -> Element {
    let solana_context =
        use_context::<SolanaContext>(cx).expect("AccountInfo must be used within a SolanaProvider");

    let account_info = use_account_info(cx, &solana_context.client, pubkey);

    match &*account_info.read() {
        Some(Ok(Some(account))) => {
            rsx! {
                div { class: "account-info",
                    h3 { "Account Information" }
                    div { class: "info-row",
                        span { class: "label", "Public Key:" }
                        span { class: "value", "{pubkey}" }
                    }
                    div { class: "info-row",
                        span { class: "label", "Balance:" }
                        span { class: "value", "{} lamports", account.lamports }
                    }
                    div { class: "info-row",
                        span { class: "label", "Owner:" }
                        span { class: "value", "{account.owner}" }
                    }
                    div { class: "info-row",
                        span { class: "label", "Executable:" }
                        span { class: "value", "{}", account.executable }
                    }
                    div { class: "info-row",
                        span { class: "label", "Data Size:" }
                        span { class: "value", "{} bytes", account.data.len() }
                    }
                    div { class: "info-row",
                        span { class: "label", "Rent Epoch:" }
                        span { class: "value", "{}", account.rent_epoch }
                    }
                }
            }
        }
        Some(Ok(None)) => {
            rsx! {
                div { class: "account-not-found",
                    p { "Account not found: {pubkey}" }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div { class: "account-error",
                    p { "Error fetching account info: {e}" }
                }
            }
        }
        None => {
            rsx! {
                div { class: "account-loading",
                    p { "Loading account information..." }
                }
            }
        }
    }
}

#[cfg(feature = "dioxus")]
/// Component to display network information
#[component]
pub fn NetworkInfo(cx: Scope) -> Element {
    let solana_context =
        use_context::<SolanaContext>(cx).expect("NetworkInfo must be used within a SolanaProvider");

    let blockhash = use_latest_blockhash(cx, &solana_context.client);

    rsx! {
        div { class: "network-info",
            h3 { "Network Information" }
            div { class: "info-row",
                span { class: "label", "Network:" }
                span { class: "value", "{:?}", solana_context.network }
            }
            div { class: "info-row",
                span { class: "label", "Endpoint:" }
                span { class: "value", "{}", solana_context.network.endpoint() }
            }
            match &*blockhash.read() {
                Some(Ok(latest)) => rsx! {
                    div { class: "info-row",
                        span { class: "label", "Latest Blockhash:" }
                        span { class: "value", "{}", latest.blockhash }
                    }
                    div { class: "info-row",
                        span { class: "label", "Valid Until:" }
                        span { class: "value", "Block {}", latest.last_valid_block_height }
                    }
                }
                Some(Err(e)) => rsx! {
                    div { class: "info-row error",
                        span { class: "label", "Blockhash Error:" }
                        span { class: "value", "{e}" }
                    }
                }
                None => rsx! {
                    div { class: "info-row",
                        span { class: "label", "Blockhash:" }
                        span { class: "value", "Loading..." }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "dioxus")]
/// Component for connecting to different networks
#[component]
pub fn NetworkSelector(
    cx: Scope,
    on_network_change: EventHandler<gloo_solana::Network>,
) -> Element {
    let solana_context = use_context::<SolanaContext>(cx);
    let current_network = solana_context.map(|ctx| ctx.network.clone());

    rsx! {
        div { class: "network-selector",
            h3 { "Select Network" }
            select {
                onchange: move |evt| {
                    let network = match evt.value.as_str() {
                        "mainnet" => gloo_solana::Network::Mainnet,
                        "devnet" => gloo_solana::Network::Devnet,
                        "testnet" => gloo_solana::Network::Testnet,
                        "surfpool" => gloo_solana::surfpool_network(),
                        _ => gloo_solana::Network::Devnet,
                    };
                    on_network_change.call(network);
                },
                option {
                    value: "mainnet",
                    selected: current_network.as_ref().map_or(false, |n| matches!(n, gloo_solana::Network::Mainnet)),
                    "Mainnet Beta"
                }
                option {
                    value: "devnet",
                    selected: current_network.as_ref().map_or(false, |n| matches!(n, gloo_solana::Network::Devnet)),
                    "Devnet"
                }
                option {
                    value: "testnet",
                    selected: current_network.as_ref().map_or(false, |n| matches!(n, gloo_solana::Network::Testnet)),
                    "Testnet"
                }
                option {
                    value: "surfpool",
                    selected: current_network.as_ref().map_or(false, |n| matches!(n, gloo_solana::Network::Custom(_))),
                    "Surfpool (Local)"
                }
            }
        }
    }
}

// Re-export components when dioxus feature is enabled
#[cfg(feature = "dioxus")]
pub use {
    use_account_info, use_balance, use_latest_blockhash, use_solana_client, AccountInfo,
    BalanceDisplay, NetworkInfo, NetworkSelector, SolanaContext, SolanaProvider,
};

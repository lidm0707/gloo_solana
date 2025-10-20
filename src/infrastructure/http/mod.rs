//! HTTP client infrastructure for WASM and native environments
//!
//! This module provides abstractions and implementations for making HTTP requests
//! from both WASM environments using gloo_net and native environments using reqwest,
//! with a focus on Solana JSON-RPC calls.

#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;
use thiserror::Error;

/// HTTP client trait for abstraction over different HTTP implementations
pub trait HttpClient {
    /// Send a POST request with JSON body
    fn post_json<'a, Req, Resp>(
        &'a self,
        url: &'a str,
        body: &'a Req,
    ) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + 'static;

    /// Send a GET request
    fn get<'a, Resp>(&'a self, url: &'a str) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Resp: for<'de> Deserialize<'de> + 'static;
}

/// WASM-compatible HTTP client implementation using gloo_net
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct WasmHttpClient {
    _marker: std::marker::PhantomData<()>,
}

#[cfg(target_arch = "wasm32")]
impl WasmHttpClient {
    /// Create a new WASM HTTP client
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a new WASM HTTP client with custom headers
    pub fn with_headers(_headers: Vec<(String, String)>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for WasmHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
impl HttpClient for WasmHttpClient {
    /// Send a POST request with JSON body
    fn post_json<'a, Req, Resp>(
        &'a self,
        url: &'a str,
        body: &'a Req,
    ) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + 'static,
    {
        async move {
            let json_body = serde_json::to_string(body)
                .map_err(|e| HttpError::SerializationError(e.to_string()))?;

            let request = Request::post(url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .body(&json_body)
                .map_err(|e| HttpError::RequestError(e.to_string()))?;

            let response = request
                .send()
                .await
                .map_err(|e| HttpError::RequestError(e.to_string()))?;

            if !response.ok() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(HttpError::HttpStatusError {
                    status,
                    message: error_text,
                });
            }

            let response_text = response
                .text()
                .await
                .map_err(|e| HttpError::ResponseError(e.to_string()))?;

            serde_json::from_str(&response_text)
                .map_err(|e| HttpError::DeserializationError(e.to_string()))
        }
    }

    /// Send a GET request
    fn get<'a, Resp>(&'a self, url: &'a str) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Resp: for<'de> Deserialize<'de> + 'static,
    {
        async move {
            let response = Request::get(url)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| HttpError::RequestError(e.to_string()))?;

            if !response.ok() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(HttpError::HttpStatusError {
                    status,
                    message: error_text,
                });
            }

            let response_text = response
                .text()
                .await
                .map_err(|e| HttpError::ResponseError(e.to_string()))?;

            serde_json::from_str(&response_text)
                .map_err(|e| HttpError::DeserializationError(e.to_string()))
        }
    }
}

/// Native HTTP client implementation using reqwest
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct NativeHttpClient {
    client: Client,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHttpClient {
    /// Create a new native HTTP client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Create a new native HTTP client with custom headers
    pub fn with_headers(headers: Vec<(String, String)>) -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();

        for (key, value) in headers {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                reqwest::header::HeaderValue::from_str(&value),
            ) {
                default_headers.insert(name, val);
            }
        }

        let client = Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for NativeHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient for NativeHttpClient {
    /// Send a POST request with JSON body
    fn post_json<'a, Req, Resp>(
        &'a self,
        url: &'a str,
        body: &'a Req,
    ) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + 'static,
    {
        async move {
            let response = self
                .client
                .post(url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .json(body)
                .send()
                .await
                .map_err(|e| HttpError::RequestError(e.to_string()))?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(HttpError::HttpStatusError {
                    status,
                    message: error_text,
                });
            }

            let response_text = response
                .text()
                .await
                .map_err(|e| HttpError::ResponseError(e.to_string()))?;

            serde_json::from_str(&response_text)
                .map_err(|e| HttpError::DeserializationError(e.to_string()))
        }
    }

    /// Send a GET request
    fn get<'a, Resp>(&'a self, url: &'a str) -> impl Future<Output = Result<Resp, HttpError>> + 'a
    where
        Resp: for<'de> Deserialize<'de> + 'static,
    {
        async move {
            let response = self
                .client
                .get(url)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| HttpError::RequestError(e.to_string()))?;

            if !response.status().is_success() {
                let status = response.status().as_u16();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(HttpError::HttpStatusError {
                    status,
                    message: error_text,
                });
            }

            let response_text = response
                .text()
                .await
                .map_err(|e| HttpError::ResponseError(e.to_string()))?;

            serde_json::from_str(&response_text)
                .map_err(|e| HttpError::DeserializationError(e.to_string()))
        }
    }
}

/// Errors that can occur during HTTP operations
#[derive(Debug, Clone, Error)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestError(String),

    #[error("HTTP status error {status}: {message}")]
    HttpStatusError { status: u16, message: String },

    #[error("Response processing error: {0}")]
    ResponseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = NativeHttpClient::new();
        // Just test that it creates without panicking
        let _ = client;
    }
}

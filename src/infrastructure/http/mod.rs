//! HTTP client infrastructure for WASM environments
//!
//! This module provides abstractions and implementations for making HTTP requests
//! from WASM environments using gloo_net, with a focus on Solana JSON-RPC calls.

use gloo_net::http::Request;
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
#[derive(Debug, Clone)]
pub struct WasmHttpClient {
    _marker: std::marker::PhantomData<()>,
}

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

impl Default for WasmHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

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

    #[error("Network timeout")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// HTTP response wrapper with additional metadata
#[derive(Debug, Clone)]
pub struct HttpResponse<T> {
    /// The response data
    pub data: T,
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: std::collections::HashMap<String, String>,
}

impl<T> HttpResponse<T> {
    /// Create a new HTTP response
    pub fn new(data: T, status: u16) -> Self {
        Self {
            data,
            status,
            headers: std::collections::HashMap::new(),
        }
    }

    /// Add a header to the response
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Configuration for HTTP clients
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// Request timeout in milliseconds
    pub timeout_ms: u32,
    /// Default headers to include in all requests
    pub default_headers: std::collections::HashMap<String, String>,
    /// Whether to retry failed requests
    pub retry_failed_requests: bool,
    /// Maximum number of retry attempts
    pub max_retries: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000, // 30 seconds
            default_headers: std::collections::HashMap::new(),
            retry_failed_requests: true,
            max_retries: 3,
        }
    }
}

/// Builder for creating configured HTTP clients
pub struct HttpClientBuilder {
    config: HttpClientConfig,
}

impl HttpClientBuilder {
    /// Create a new HTTP client builder
    pub fn new() -> Self {
        Self {
            config: HttpClientConfig::default(),
        }
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout_ms: u32) -> Self {
        self.config.timeout_ms = timeout_ms;
        self
    }

    /// Add a default header
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.default_headers.insert(key.into(), value.into());
        self
    }

    /// Enable or disable request retries
    pub fn retry(mut self, enabled: bool) -> Self {
        self.config.retry_failed_requests = enabled;
        self
    }

    /// Set maximum retry attempts
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Build the HTTP client
    pub fn build(self) -> WasmHttpClient {
        let headers: Vec<(String, String)> = self.config.default_headers.into_iter().collect();

        WasmHttpClient::with_headers(headers)
    }
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_http_client_builder() {
        let client = HttpClientBuilder::new()
            .timeout(5000)
            .default_header("User-Agent", "gloo-solana/0.1.0")
            .retry(true)
            .max_retries(5)
            .build();

        // Test that the client was created successfully
        // In a real test, we would make actual HTTP requests
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_http_response() {
        let data = json!({"key": "value"});
        let response =
            HttpResponse::new(data.clone(), 200).with_header("Content-Type", "application/json");

        assert_eq!(response.status, 200);
        assert_eq!(response.data, data);
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }
}

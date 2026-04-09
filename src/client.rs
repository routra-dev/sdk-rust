use reqwest::{header, Client};

use crate::types::{ChatRequest, ChatResponse};

const BASE_URL: &str = "https://api.routra.dev/v1";

/// Routra async HTTP client.
///
/// Drop-in equivalent for the OpenAI client - just change the API key and base URL.
/// Set a `policy` to apply routing constraints (cheapest, balanced, gdpr-eu, ...).
///
/// The base URL defaults to `https://api.routra.dev/v1` but can be overridden
/// via [`with_base_url`](Self::with_base_url) or the `ROUTRA_BASE_URL` env var.
pub struct Routra {
    client: Client,
    api_key: String,
    base_url: String,
    policy: Option<String>,
}

impl Routra {
    /// Create a new client with the given API key.
    ///
    /// The base URL is resolved from `ROUTRA_BASE_URL` env var (if set),
    /// otherwise defaults to `https://api.routra.dev/v1`.
    pub fn new(api_key: impl Into<String>) -> Self {
        let base_url = std::env::var("ROUTRA_BASE_URL")
            .unwrap_or_else(|_| BASE_URL.to_string());
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url,
            policy: None,
        }
    }

    /// Override the base URL (useful for local dev / testing).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Return a new client with the given policy applied.
    /// Sets `X-Routra-Policy` on every request made by the returned client.
    pub fn with_policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    /// POST /v1/chat/completions
    pub async fn chat_completions(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, reqwest::Error> {
        let url = format!("{}/chat/completions", self.base_url);
        let mut builder = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&req);

        if let Some(policy) = &self.policy {
            builder = builder.header("X-Routra-Policy", policy);
        }

        let resp = builder.send().await?.error_for_status()?;
        resp.json::<ChatResponse>().await
    }
}

use reqwest::{header, Client};

use crate::types::{
    ChatRequest, ChatResponse, EmbeddingRequest, EmbeddingResponse, ImageRequest, ImageResponse,
    SpeechRequest, TranscriptionResponse,
};
use crate::management::ManagementClient;

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
        let base_url =
            std::env::var("ROUTRA_BASE_URL").unwrap_or_else(|_| BASE_URL.to_string());
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

    /// Build a request with auth and policy headers.
    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self
            .client
            .request(method, &url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json");

        if let Some(policy) = &self.policy {
            builder = builder.header("X-Routra-Policy", policy);
        }
        builder
    }

    /// POST /v1/chat/completions
    pub async fn chat_completions(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, reqwest::Error> {
        self.request(reqwest::Method::POST, "/chat/completions")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<ChatResponse>()
            .await
    }

    /// POST /v1/embeddings
    pub async fn embeddings(
        &self,
        req: EmbeddingRequest,
    ) -> Result<EmbeddingResponse, reqwest::Error> {
        self.request(reqwest::Method::POST, "/embeddings")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<EmbeddingResponse>()
            .await
    }

    /// POST /v1/images/generations
    pub async fn image_generate(
        &self,
        req: ImageRequest,
    ) -> Result<ImageResponse, reqwest::Error> {
        self.request(reqwest::Method::POST, "/images/generations")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<ImageResponse>()
            .await
    }

    /// POST /v1/audio/speech — returns raw audio bytes.
    pub async fn speech(&self, req: SpeechRequest) -> Result<bytes::Bytes, reqwest::Error> {
        self.request(reqwest::Method::POST, "/audio/speech")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await
    }

    /// POST /v1/audio/transcriptions — multipart file upload.
    pub async fn transcribe(
        &self,
        file: Vec<u8>,
        filename: impl Into<String>,
        model: impl Into<String>,
    ) -> Result<TranscriptionResponse, reqwest::Error> {
        let url = format!("{}/audio/transcriptions", self.base_url);
        let part = reqwest::multipart::Part::bytes(file).file_name(filename.into());
        let form = reqwest::multipart::Form::new()
            .text("model", model.into())
            .part("file", part);

        let mut builder = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .multipart(form);

        if let Some(policy) = &self.policy {
            builder = builder.header("X-Routra-Policy", policy);
        }

        builder
            .send()
            .await?
            .error_for_status()?
            .json::<TranscriptionResponse>()
            .await
    }

    /// Return a [`ManagementClient`] that shares the same API key and base URL.
    pub fn management(&self) -> ManagementClient {
        let mgmt_base = self.base_url.trim_end_matches("/v1").trim_end_matches('/');
        ManagementClient::new(&self.api_key, mgmt_base)
    }
}

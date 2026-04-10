use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".into(), content: content.into() }
    }
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: "system".into(), content: content.into() }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    /// Routing metadata injected by Routra proxy.
    pub routra: Option<RoutingMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoutingMetadata {
    pub provider: String,
    pub latency_ms: u32,
    pub score: f64,
    pub cost_usd: Option<f64>,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    /// Non-token pricing unit: "images", "characters", "seconds".
    pub usage_unit: Option<String>,
    /// Non-token usage value (e.g. image count, character count, duration).
    pub usage_value: Option<f64>,
    pub failover: Option<bool>,
    pub ttfb_ms: Option<u32>,
}

// ── Embeddings ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: EmbeddingInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for EmbeddingInput {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Option<EmbeddingUsage>,
    pub routra: Option<RoutingMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingData {
    pub index: u32,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

// ── Image Generation ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Default)]
pub struct ImageRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
    pub routra: Option<RoutingMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageData {
    pub url: Option<String>,
    pub b64_json: Option<String>,
    pub revised_prompt: Option<String>,
}

// ── Text-to-Speech ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Default)]
pub struct SpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

// ── Speech-to-Text ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
    pub duration: Option<f64>,
    pub routra: Option<RoutingMetadata>,
}

//! Routra Rust SDK
//!
//! Thin async HTTP client wrapping the Routra API.
//! Presents an OpenAI-compatible interface with routing metadata on every response.
//!
//! # Example
//! ```no_run
//! use routra::{Routra, ChatRequest, Message};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Routra::new("rtr-...")
//!         .with_policy("cheapest");
//!
//!     let resp = client.chat_completions(ChatRequest {
//!         model: "auto".into(),
//!         messages: vec![Message::user("Hello")],
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("{}", resp.choices[0].message.content);
//!     if let Some(meta) = resp.routra {
//!         println!("provider={} score={:.4}", meta.provider, meta.score);
//!     }
//!     Ok(())
//! }
//! ```

mod client;
mod types;
pub mod management;

pub use client::Routra;
pub use types::{
    ChatRequest, ChatResponse, Choice, EmbeddingData, EmbeddingInput, EmbeddingRequest,
    EmbeddingResponse, EmbeddingUsage, ImageData, ImageRequest, ImageResponse, Message,
    RoutingMetadata, SpeechRequest, TranscriptionResponse, Usage,
};
pub use management::ManagementClient;

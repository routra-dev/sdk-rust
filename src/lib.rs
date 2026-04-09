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

pub use client::Routra;
pub use types::{ChatRequest, ChatResponse, Choice, Message, RoutingMetadata, Usage};

/// Generated management API client (keys, policies, usage, billing, etc.).
///
/// Enable with the `management` feature flag:
/// ```toml
/// routra = { version = "0.1", features = ["management"] }
/// ```
/// The client is generated at build time from the public Routra OpenAPI spec
/// via `progenitor`. Add the `contracts` git submodule or set `CONTRACTS_SPEC`
/// to point at `contracts/openapi/routra.yaml` before building.
#[cfg(feature = "management")]
pub mod management {
    include!(concat!(env!("OUT_DIR"), "/management.rs"));
}

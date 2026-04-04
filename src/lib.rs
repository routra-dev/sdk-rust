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
//!         println!("provider={} cost=${:.6}", meta.provider, meta.cost_usd);
//!     }
//!     Ok(())
//! }
//! ```

mod client;
mod types;

pub use client::Routra;
pub use types::{ChatRequest, ChatResponse, Choice, Message, RoutingMetadata, Usage};

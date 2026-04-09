# Routra Rust SDK

Async HTTP client for the Routra API. Presents an OpenAI-compatible interface with typed routing metadata.

## Installation

```toml
[dependencies]
routra-sdk = "0.1"
tokio = { version = "1", features = ["rt", "macros"] }
```

To also use the management API (keys, policies, usage, billing):

```toml
routra-sdk = { version = "0.1", features = ["management"] }
```

## Quick Start

```rust
use routra::{Routra, ChatRequest, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Routra::new("rtr-...")
        .with_policy("cheapest");

    let resp = client.chat_completions(ChatRequest {
        model: "auto".into(),
        messages: vec![Message::user("Hello")],
        ..Default::default()
    }).await?;

    println!("{}", resp.choices[0].message.content);
    if let Some(meta) = &resp.routra {
        println!("provider={} score={:.4}", meta.provider, meta.score);
        if let Some(cost) = meta.cost_usd {
            println!("cost=${:.6}", cost);
        }
    }
    Ok(())
}
```

## Routing Policies

```rust
// Set a default policy
let client = Routra::new("rtr-...").with_policy("cheapest");

// Override base URL for local dev
let client = Routra::new("rtr-...").with_base_url("http://localhost:8080/v1");
```

## Routing Metadata

Every non-streaming response includes optional `RoutingMetadata`:

| Field | Type | Description |
|-------|------|-------------|
| `provider` | `String` | Provider slug that served the request |
| `latency_ms` | `u32` | Total provider response time in milliseconds |
| `score` | `f64` | Normalized routing score (0–1) |
| `cost_usd` | `Option<f64>` | Estimated cost in USD |
| `input_tokens` | `Option<i32>` | Input token count |
| `output_tokens` | `Option<i32>` | Output token count |
| `failover` | `Option<bool>` | Whether the request was rerouted after a provider failure |
| `ttfb_ms` | `Option<u32>` | Time to first byte |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ROUTRA_BASE_URL` | Override API base URL (default: `https://api.routra.dev/v1`) |

## Management Feature Flag

When built with `features = ["management"]`, the SDK includes a fully typed management API client generated from the OpenAPI spec via [progenitor](https://github.com/oxidecomputer/progenitor):

```rust
use routra::management;
// Keys, policies, usage, billing, batch endpoints
```

This requires the `contracts/openapi/routra.yaml` spec to be available at build time. See [build.rs](build.rs) for spec resolution.

## Error Handling

All methods return `Result<T, reqwest::Error>`. Errors include network failures, HTTP error status codes, and JSON deserialization failures:

```rust
match client.chat_completions(req).await {
    Ok(resp) => println!("{}", resp.choices[0].message.content),
    Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
        eprintln!("Invalid API key");
    }
    Err(e) => eprintln!("Request failed: {e}"),
}
```

## License

MIT

//! Build script — generates a typed management API client from the Routra OpenAPI spec
//! using `progenitor`.
//!
//! Spec resolution order (first wins):
//!   1. `CONTRACTS_SPEC` env var — point at any local or remote path
//!   2. `./contracts/openapi/routra.yaml` — when `contracts` is a git submodule
//!
//! To add contracts as a submodule (one-time, in sdk-rust repo root):
//!   git submodule add https://github.com/routra/contracts contracts
//!   git submodule update --init
//!
//! Then `cargo build` will generate `src/management.rs` automatically.

fn main() {
    // Only regenerate when spec changes (not on every cargo check).
    println!("cargo:rerun-if-env-changed=CONTRACTS_SPEC");
    println!("cargo:rerun-if-changed=contracts/openapi/routra.yaml");

    // Resolve spec path.
    let spec_path = if let Ok(env_path) = std::env::var("CONTRACTS_SPEC") {
        let p = std::path::PathBuf::from(&env_path);
        if !p.exists() {
            panic!(
                "CONTRACTS_SPEC={env_path} does not exist. \
                 Check the path or unset the env var to use the submodule."
            );
        }
        p
    } else {
        let p = std::path::PathBuf::from("contracts/openapi/routra.yaml");
        if !p.exists() {
            // Spec not available — skip codegen, the committed generated file
            // (if any) will be used. This allows `cargo build` without the
            // submodule for users who only need the proxy/chat APIs.
            println!(
                "cargo:warning=contracts/openapi/routra.yaml not found — \
                 skipping management API codegen. \
                 Run: git submodule update --init contracts"
            );
            return;
        }
        p
    };

    let spec_str = std::fs::read_to_string(&spec_path)
        .expect("failed to read OpenAPI spec");

    let spec = serde_json::from_str::<serde_json::Value>(
        &serde_yaml::from_str::<serde_json::Value>(&spec_str)
            .expect("invalid YAML in spec")
            .to_string(),
    )
    .expect("failed to reserialize spec as JSON");

    let mut generator = progenitor::Generator::default();
    let tokens = generator
        .generate_tokens(&spec)
        .expect("progenitor failed to generate client");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_file = out_dir.join("management.rs");
    std::fs::write(&out_file, tokens.to_string()).expect("failed to write generated client");

    // Optionally pretty-print with rustfmt if available.
    let _ = std::process::Command::new("rustfmt")
        .arg(&out_file)
        .status();
}

//! `gfs` – Guepard data-plane CLI binary.
//!
//! Thin wrapper around the library. See `gfs_cli::run()` for programmatic use.

use gfs_cli::output::red;
use serde_json::json;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let wants_json = args.iter().any(|a| a == "--json");

    match gfs_cli::run(args).await {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(err) => {
            if wants_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "error": {
                            "message": err.to_string(),
                            "details": format!("{err:#}"),
                        }
                    }))
                    .unwrap_or_else(|_| "{\"error\":{\"message\":\"serialization failed\"}}".into())
                );
            } else {
                eprintln!("{} {err:#}", red("error:"));
            }
            std::process::exit(1);
        }
    }
}

#![allow(unused)]
use eyre::Result;
use is_terminal::IsTerminal;
pub use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

pub fn init_log(default_level: &str) -> Result<()> {
    // Create an EnvFilter that first checks for the RUST_LOG env var
    // If not found, fallback to the provided default_level
    let env_filter =
        EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new(default_level));

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        // .with_thread_ids(true)
        // .with_thread_names(true)
        // .with_file(true)
        // .with_line_number(true)
        .with_ansi(std::io::stderr().is_terminal())
        .compact()
        // .json()
        // .flatten_event(true)
        .init();
    Ok(())
}

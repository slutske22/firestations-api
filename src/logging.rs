//! Logging

use tracing_subscriber::{filter::LevelFilter, EnvFilter};

/// Initializes the tracing subsystem.
#[inline]
pub fn initialize_tracing(filter: Option<&str>) {
    let tracing_subscriber = tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(
                    filter
                        .and_then(|f| f.parse().ok())
                        .unwrap_or_else(|| LevelFilter::INFO.into()),
                )
                .from_env_lossy(),
        )
        .with_filter_reloading();
    tracing_subscriber.init();
}

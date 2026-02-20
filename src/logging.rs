use std::env;

use anyhow::Result;
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging() -> Result<non_blocking::WorkerGuard> {
    let console_layer = fmt::layer()
        .with_target(false)
        .with_file(true)
        .with_level(true);

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("agent")
        .filename_suffix("log")
        .build("logs")?;

    let (non_blocking_writer, worker_guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking_writer)
        .with_target(false)
        .with_level(true);

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info,agent=info,rig=warn".into()),
        ))
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(worker_guard)
}

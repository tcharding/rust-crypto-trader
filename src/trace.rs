use anyhow::Result;
use atty::{self, Stream};
use log::LevelFilter;
use tracing::{info, subscriber, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

// This crate tracing level => use `LevelFilter`
//
// LevelFilter::Error
// LevelFilter::Warn
// LevelFilter::Info
// LevelFilter::Debug
// LevelFilter::Trace
//
// Upstream lib tracing level => use `Level`
//
// Level::ERROR,
// Level::WARN,
// Level::INFO,
// Level::DEBUG,
// Level::TRACE,

pub fn init_tracing() -> Result<()> {
    let crate_level = LevelFilter::Debug;
    let lib_level = Level::INFO;

    LogTracer::init_with_filter(crate_level)?;

    let ansi = atty::is(Stream::Stdout);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(lib_level.clone())
        .with_ansi(ansi)
        .finish();

    subscriber::set_global_default(subscriber)?;
    info!(
        "Initialized tracing with crate level: {}, upstream lib level: {}",
        crate_level, lib_level
    );

    Ok(())
}

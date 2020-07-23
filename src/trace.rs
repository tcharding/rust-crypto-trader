use anyhow::Result;
use atty::{self, Stream};
use log::LevelFilter;
use tracing::{info, subscriber, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

pub fn init_tracing(crate_level: LevelFilter) -> Result<()> {
    if crate_level == LevelFilter::Off {
        return Ok(());
    }

    let lib_level = LevelFilter::Info;

    LogTracer::init_with_filter(lib_level)?;

    let ansi = atty::is(Stream::Stdout);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(into_level(crate_level))
        .with_ansi(ansi)
        .finish();

    subscriber::set_global_default(subscriber)?;
    info!("Initialized tracing with level: {}", crate_level);

    Ok(())
}

fn into_level(level: LevelFilter) -> Level {
    match level {
        LevelFilter::Off => unreachable!(),
        LevelFilter::Error => Level::ERROR,
        LevelFilter::Warn => Level::WARN,
        LevelFilter::Info => Level::INFO,
        LevelFilter::Debug => Level::DEBUG,
        LevelFilter::Trace => Level::TRACE,
    }
}

use anyhow::{Context, Result};
use log::LevelFilter;

use crypto_trader::{bot::spread, config, trace};

/// Crypto-trader configuration file.
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
        .expect("failed to construct config path");

    trace::init_tracing(LevelFilter::Trace)?;

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    spread::run(config.keys.read).await; // Never returns.

    Ok(())
}

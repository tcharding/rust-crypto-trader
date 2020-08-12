use anyhow::{Context, Result};
use crypto_trader::{
    config,
    market::{self, Market},
    trace,
};

/// Crypto-trader configuration file.
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
        .expect("failed to construct config path");

    trace::init_tracing()?;

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    market::test_ir_api(config.keys.clone()).await;

    let m = Market::default().with_read_only(config.keys.read);

    let orderbook = m.order_book().await?;
    println!("{}", orderbook.bid_ask_spread());

    Ok(())
}

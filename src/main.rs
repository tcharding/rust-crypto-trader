use anyhow::{Context, Result};
use crypto_trader::{api::market, config, Api};

/// Crypto-trader configuration file.
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
        .expect("failed to construct config path");

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    market::test_api(config.keys.clone()).await;

    let api = Api::new(config.keys);
    let orderbook = api.order_book().await?;

    let s = orderbook.bid_offer_spread();

    println!("Current bid/offer spread: {}", s);

    Ok(())
}

use anyhow::{Context, Result};
use crypto_trader::{config, market::*};

/// Crypto-trader configuration file.
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
        .expect("failed to construct config path");

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    let key = config.keys.read.key;
    let secret = config.keys.read.secret;

    assert_public_api().await?;
    assert_private_api(&key, &secret).await?;

    Ok(())
}

async fn assert_public_api() -> Result<()> {
    let api = Public::default();
    let mut orderbook = api.get_order_book("Xbt", "Aud").await?;

    let s = orderbook.bid_offer_spread();
    println!("Current bid/offer spread: {}", s);

    Ok(())
}

async fn assert_private_api(key: &str, secret: &str) -> Result<()> {
    let mut api = Private::new(key, secret);
    let _ = api.get_open_orders("Xbt", "Aud", 1).await?;
    let _ = api.get_closed_orders("Xbt", "Aud", 1).await?;

    Ok(())
}

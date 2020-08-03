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

    let key = config.keys.read;
    can_get_orderbook().await; // Test public API.
    can_get_orders(&key).await; // Test private API.

    let spread = rate_and_spread().await?;
    println!("current minimum spread: {}", spread);

    Ok(())
}

// Test function, cannot be a unit test because we need an API key.
async fn can_get_orders(key: &str) {
    let api = Private::new(key);
    let _ = api.get_open_orders().await.expect("API call failed");
    // tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    // let _ = api.get_closed_orders().await.expect("API call failed");
}

async fn can_get_orderbook() {
    let api = Public::default();
    let _ = api
        .get_order_book("Xbt", "Aud")
        .await
        .expect("API call failed");
}

async fn rate_and_spread() -> Result<String> {
    let api = Public::default();
    let mut orderbook = api
        .get_order_book("Xbt", "Aud")
        .await
        .expect("API call failed");

    let s = orderbook.bid_offer_spread();

    Ok(s)
}

use anyhow::{Context, Result};
use log::LevelFilter;
use rust_decimal::Decimal;

use crypto_trader::{
    config,
    market::{kraken, Market},
    num, trace,
};

/// Crypto-trader configuration files (we pre-pend HOME to these).
const IR_CONFIG_FILE: &str = ".config/crypto-trader/config.toml";
const KRAKEN_CONFIG_FILE: &str = ".config/crypto-trader/kraken.json";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(IR_CONFIG_FILE))
        .expect("failed to construct config path");

    trace::init_tracing(LevelFilter::Trace)?;

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    // market::test_ir_api(config.keys.clone()).await;
    // spread::run(config.keys.read).await; // Never returns.

    let m = Market::default().with_read_only(config.keys.read);

    let orderbook = m.order_book().await?;
    let (bid, ask) = orderbook.spread_to_fill(Decimal::from(1))?;
    let (spread, percent) = num::spread_percent(&bid, &ask);

    println!(
        "{} {}",
        num::to_aud_string(&spread),
        num::to_percent_string(&percent)
    );

    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(KRAKEN_CONFIG_FILE))
        .expect("failed to construct config path");
    let mut kapi = kraken::Api::new(path).expect("failed to create kraken API");
    kapi.assert_public()
        .expect("failed to assert kraken API works");

    Ok(())
}

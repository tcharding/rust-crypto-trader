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

    // Assert the entire API works.
    assert_public_api_methods().await?;
    assert_private_api_all_read_only(&key, &secret).await?;

    current_bid_offer_spread().await?;

    Ok(())
}

/// Call each of the public API methods.
async fn assert_public_api_methods() -> Result<()> {
    let base = "Xbt";
    let quote = "Aud";

    let api = Public::default();

    let _ = api.get_valid_primary_currency_codes().await?;
    let _ = api.get_valid_secondary_currency_codes().await?;
    let _ = api.get_valid_limit_order_types().await?;
    let _ = api.get_valid_market_order_types().await?;
    let _ = api.get_valid_order_types().await?;
    let _ = api.get_valid_transaction_types().await?;
    let _ = api.get_market_summary(base, quote).await?;
    let _ = api.get_order_book(base, quote).await?;
    let _ = api.get_all_orders(base, quote).await?;
    let _ = api.get_trade_history_summary(base, quote, 1).await?;
    let _ = api.get_recent_trades(base, quote, 10).await?;
    let _ = api.get_fx_rates().await?;

    Ok(())
}

/// Call each of the private API methods that require a read-only key.
async fn assert_private_api_all_read_only(key: &str, secret: &str) -> Result<()> {
    let base = "Xbt";
    let quote = "Aud";
    let index = 1;

    let mut api = Private::new(key, secret);

    let _ = api.get_open_orders(base, quote, index).await?;
    let _ = api.get_closed_orders(base, quote, index).await?;
    let _ = api.get_closed_filled_orders(base, quote, index).await?;
    // let _ = api.get_order_details(order_guuid).await.?;
    let _ = api.get_accounts().await?;
    // let _ = api.get_transactions().await.?;
    let _ = api.get_digital_currency_deposit_address(base).await?;
    let _ = api
        .get_digital_currency_deposit_addresses(base, index)
        .await?;
    let _ = api.get_trades(index).await?;
    let _ = api.get_brokerage_fees().await?;
    // let _ = api.get_digital_currency_withdrawal(tx_guid).await.?;

    Ok(())
}

#[allow(dead_code)]
/// Call each of the private API methods that require an admin key.
async fn assert_private_api_all_admin(_key: &str, _secret: &str) -> Result<()> {
    // let _ = api.sync_digital_currency_deposit_address_with_blockchain().await.?;
    // let _ = api.place_limit_order().await.?;
    // let _ = api.place_market_order().await.?;
    // let _ = api.cancel_order().await.?;
    // let _ = api.request_fiat_withdrawal().await.?;
    unimplemented!()
}

#[allow(dead_code)]
/// Call each of the private API methods that require a full access key.
async fn assert_private_api_all_full_access(_key: &str, _secret: &str) -> Result<()> {
    // let _ = api.withdraw_digital_currency().await.?;
    unimplemented!()
}

async fn current_bid_offer_spread() -> Result<()> {
    let api = Public::default();
    let mut orderbook = api.get_order_book("Xbt", "Aud").await?;

    let s = orderbook.bid_offer_spread();
    println!("Current bid/offer spread: {}", s);

    Ok(())
}

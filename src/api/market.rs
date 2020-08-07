//! Implements the API for Inedependent Reserve crypto exchange.
//!
//! Private methods require authentication using an API key, signature, and a
//! nonce.

mod private;
mod public;

pub use private::*;
pub use public::*;

// Authentication
//
// All private API methods require authentication. All method
// parameters (except signature) are required to authenticate a
// request. There are three additional parameters which should be
// passed to private API methods:
// - API Key
// - Nonce
// - Signature

// API key
//
// To generate an API Key, go to the Settings page, click "API
// Keys" and then click "generate". Select the level of access to
// grant the key and reenter your password to confirm the creation of
// the key. Ensure that you select the lowest level of access required
// for your usage, the recommended level being Read-only.

// Nonce
//
// The nonce is a 64 bit unsigned integer. The nonce must
// increase with each request made to the API.

// Example: If the nonce is set to 1 in the first request, it must be
// set to at least 2 in the subsequent request. It is not necessary to
// start with 1. A common practice is to use unix time for this
// parameter.

// Signature
//
// Signature is a HMAC-SHA256 encoded message. The message
// is comma-separated string containing the API method URL, and a
// comma separated list of all method parameters (except signature) in
// the form: "parameterName=parameterValue". The HMAC-SHA256 code must
// be generated using the API Secret that was generated with your API
// key. This code must be converted to it's hexadecimal
// representation.

use crate::config::{Key, Keys};
use anyhow::Result;

pub async fn test_api(keys: Keys) {
    assert_public_api()
        .await
        .expect("public API method assertion failed");
    assert_private_api_read_only(keys.read)
        .await
        .expect("private read-only API assertion failed");
}

/// Call each of the public API methods.
async fn assert_public_api() -> Result<()> {
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
async fn assert_private_api_read_only(read: Key) -> Result<()> {
    let base = "Xbt";
    let quote = "Aud";
    let index = 1;
    let nonce = crate::nonce();

    let mut api = Private::new(nonce, read.key, read.secret);

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

//! This module wraps API access to the Independent Reserve Exchange.
//!
//! Here we have convenience functions for accessing the API for trading pair
//! BTC/AUD. Also logic and data structures for manipulating the raw data
//! returned from the exchange. The raw API methods are implemented in the `api`
//! module.

#[allow(dead_code)] // Don't warn if we do not use all the API methods.
mod api;
pub mod kraken;
mod orderbook;

use self::api::{Private, Public};
use crate::Key;
use anyhow::Result;

pub use orderbook::*;
pub use test::*;

/// Primary currency (base).
const PRI: &str = "Xbt";
/// Secondary currency (quote).
const SEC: &str = "Aud";

#[derive(Clone, Debug)]
pub struct Market {
    public: Public,
    private: Option<Private>,
}

impl Market {
    pub fn with_read_only(self, read: Key) -> Self {
        let nonce = crate::nonce();
        let private = Private::new(nonce, read.api_key, read.api_secret);

        Market {
            public: self.public,
            private: Some(private),
        }
    }

    pub async fn order_book(&self) -> Result<OrderBook> {
        let order_book = self.public.get_order_book(PRI, SEC).await?;
        Ok(order_book.into())
    }
}

impl Default for Market {
    fn default() -> Self {
        Market {
            public: Public::default(),
            private: None,
        }
    }
}

mod test {
    use super::*;
    use crate::config::Key;
    use tracing::info;

    /// Test the Independent Reserve API.
    pub async fn test_ir_api(read_only: Key) {
        assert_public_api()
            .await
            .expect("public API method assertion failed");
        assert_private_api_read_only(read_only)
            .await
            .expect("private read-only API assertion failed");
    }

    /// Call each of the public API methods.
    async fn assert_public_api() -> Result<()> {
        let base = "Xbt";
        let quote = "Aud";

        let api = Public::default();

        info!("Running all public API methods ...");
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

        let mut api = Private::new(nonce, read.api_key, read.api_secret);

        info!("Running [most] private API methods ...");

        let _ = api.get_open_orders(base, quote, index).await?;
        let _ = api.get_closed_orders(base, quote, index).await?;

        let _ = api.get_closed_filled_orders(base, quote, index).await?;

        let _ = api.get_accounts().await?;
        let _ = api.get_digital_currency_deposit_address(base).await?;
        let _ = api
            .get_digital_currency_deposit_addresses(base, index)
            .await?;
        let _ = api.get_trades(index).await?;
        let _ = api.get_brokerage_fees().await?;

        // TODO: api.get_order_details(order_guuid).await.?;
        // TODO:  api.get_transactions().await.?;
        // TODO: api.get_digital_currency_withdrawal(tx_guid).await.?;

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
        todo!("assert_private_api_all_admin()")
    }

    #[allow(dead_code)]
    /// Call each of the private API methods that require a full access key.
    async fn assert_private_api_all_full_access(_key: &str, _secret: &str) -> Result<()> {
        // let _ = api.withdraw_digital_currency().await.?;
        todo!("implement assert_private_api_all_full_access()")
    }
}

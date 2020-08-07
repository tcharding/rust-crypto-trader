//! This module provides API access to the Independent Reserve Exchange.
//!
//! Here you will find convenience functions for accessing the API as well as
//! logic and data structures for manipulating the raw data returned from the
//! exchange. The raw API methods are implemented in the `market` module.
//!
//! At this layer we deal only with the currency pair BTC/AUD.

pub mod market;
mod orderbook;

use self::market::{Private, Public};
use crate::Keys;
use anyhow::Result;

pub use orderbook::*;

/// Primary currency (base).
const PRI: &str = "Xbt";
/// Secondary currency (quote).
const SEC: &str = "Aud";

#[derive(Clone, Debug)]
pub struct Api {
    public: Public,
    private: Private,
}

impl Api {
    pub fn new(keys: Keys) -> Self {
        let nonce = crate::nonce();
        let private = Private::new(nonce, keys.read.key, keys.read.secret);
        let public = Public::default();

        Api { public, private }
    }

    pub async fn order_book(&self) -> Result<OrderBook> {
        let order_book = self.public.get_order_book(PRI, SEC).await?;
        Ok(order_book.into())
    }
}

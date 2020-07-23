use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

// GetOpenOrders
// GetClosedOrders
// GetClosedFilledOrders
// GetOrderDetails
// GetAccounts
// GetTransactions
// GetDigitalCurrencyDepositAddress
// GetDigitalCurrencyDepositAddresses
// GetTrades
// GetBrokerageFees
// GetDigitalCurrencyWithdrawal
// PlaceLimitOrder
// PlaceMarketOrder
// CancelOrder
// SynchDigitalCurrencyDepositAddressWithBlockchain
// RequestFiatWithdrawal
// WithdrawDigitalCurrency

/// Implements the private methods for Inedependent Reserve crypto exchange API.
pub struct Private {
    client: Client,
}

impl Private {
    /// Private API URL
    const URL: &'static str = "https://api.independentreserve.com/Private";

    // HTTP POST https://api.independentreserve.com/Private/GetOpenOrders
    // {
    //     "apiKey":"{api-key}",
    //     "nonce":{nonce},
    //     "signature":"{signature}",
    //     "primaryCurrencyCode":"Xbt",
    //     "secondaryCurrencyCode":"Usd",
    //     "pageIndex":1,
    //     "pageSize":25
    // }
    pub async fn get_open_orders(
        &self,
        _base: &str,
        _quote: &str,
        _page_index: usize,
        _page_size: usize,
    ) -> Result<OpenOrders> {
        let url = self.build_url("GetOpenOrders")?;

        // TODO: Auth, sig, nonce and a POST request.

        let body = self.client.get(url).send().await?.text().await?;
        let res: OpenOrders = serde_json::from_str(&body)?;

        Ok(res)
    }

    // Build a URL from the Public API URL plus given path.
    fn build_url(&self, path: &str) -> Result<Url> {
        let s = format!("{}/{}", Self::URL, path);
        let url = Url::parse(&s)?;

        Ok(url)
    }
}

impl Default for Private {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

/// Returned by GetOpenOrders
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OpenOrders {
    page_size: usize,
    total_items: usize,
    data: Vec<OpenOrder>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OpenOrder {
    avg_price: f32,
    created_timestamp_utc: String,
    fee_percent: f32,
    order_guid: String,
    order_type: String,
    outstanding: f32,
    price: f32,
    primary_currency_code: String,
    secondary_currency_code: String,
    status: String,
    value: f32,
    volume: f32,
}

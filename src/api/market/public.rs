use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use url::Url;

// Independent Reserve Public API methods
//
// GetValidPrimaryCurrencyCodes
// GetValidSecondaryCurrencyCodes
// GetValidLimitOrderTypes
// GetValidMarketOrderTypes
// GetValidOrderTypes
// GetValidTransactionTypes
// GetMarketSummary
// GetOrderBook
// GetAllOrders
// GetTradeHistorySummary
// GetRecentTrades
// GetFxRates

/// Implements the public methods for Inedependent Reserve crypto exchange API.
#[derive(Clone, Debug)]
pub struct Public {
    client: Client,
}

impl Public {
    /// Public API URL
    const URL: &'static str = "https://api.independentreserve.com/Public";

    /// API call: GetValidPrimaryCurrencyCodes
    pub async fn get_valid_primary_currency_codes(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidPrimaryCurrencyCodes").await
    }

    /// API call: GetValidSecondaryCurrencyCodes
    pub async fn get_valid_secondary_currency_codes(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidSecondaryCurrencyCodes").await
    }
    /// API call: GetValidLimitOrderTypes
    pub async fn get_valid_limit_order_types(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidLimitOrderTypes").await
    }

    /// API call: GetValidMarketOrderTypes
    pub async fn get_valid_market_order_types(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidMarketOrderTypes").await
    }

    /// API call: GetValidOrderTypes
    pub async fn get_valid_order_types(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidOrderTypes").await
    }

    /// API call: GetValidTransactionTypes
    pub async fn get_valid_transaction_types(&self) -> Result<Vec<String>> {
        self.vec_api_call("GetValidTransactionTypes").await
    }

    /// API call: GetMarketSummary
    pub async fn get_market_summary(&self, base: &str, quote: &str) -> Result<MarketSummary> {
        let url = self.build_url("GetMarketSummary")?;

        let url = Url::parse_with_params(url.as_str(), &[
            ("primaryCurrencyCode", base),
            ("secondaryCurrencyCode", quote),
        ])?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: MarketSummary = serde_json::from_str(&body)?;

        Ok(res)
    }

    /// API call: GetOrderBook
    pub async fn get_order_book(&self, base: &str, quote: &str) -> Result<OrderBook> {
        let url = self.build_url("GetOrderBook")?;

        let url = Url::parse_with_params(url.as_str(), &[
            ("primaryCurrencyCode", base),
            ("secondaryCurrencyCode", quote),
        ])?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: OrderBook = serde_json::from_str(&body)?;

        Ok(res)
    }

    /// API call: GetAllOrders
    pub async fn get_all_orders(&self, base: &str, quote: &str) -> Result<Orders> {
        let url = self.build_url("GetAllOrders")?;

        let url = Url::parse_with_params(url.as_str(), &[
            ("primaryCurrencyCode", base),
            ("secondaryCurrencyCode", quote),
        ])?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: Orders = serde_json::from_str(&body)?;

        Ok(res)
    }

    /// API call: GetTradeHistorySummary
    pub async fn get_trade_history_summary(
        &self,
        base: &str,
        quote: &str,
        hours_past: usize,
    ) -> Result<TradeHistorySummary> {
        let url = self.build_url("GetTradeHistorySummary")?;

        let url = Url::parse_with_params(url.as_str(), &[
            ("primaryCurrencyCode", base),
            ("secondaryCurrencyCode", quote),
            ("numberOfHoursInThePastToRetrieve", &hours_past.to_string()),
        ])?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: TradeHistorySummary = serde_json::from_str(&body)?;

        Ok(res)
    }

    /// API call: GetRecentTrades
    pub async fn get_recent_trades(
        &self,
        base: &str,
        quote: &str,
        num_trades: usize,
    ) -> Result<RecentTrades> {
        let url = self.build_url("GetRecentTrades")?;

        let url = Url::parse_with_params(url.as_str(), &[
            ("primaryCurrencyCode", base),
            ("secondaryCurrencyCode", quote),
            ("numberOfRecentTradesToRetrieve", &num_trades.to_string()),
        ])?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: RecentTrades = serde_json::from_str(&body)?;

        Ok(res)
    }

    /// API call: GetFxRates
    pub async fn get_fx_rates(&self) -> Result<FxRates> {
        let url = self.build_url("GetFxRates")?;

        let body = self.client.get(url).send().await?.text().await?;
        let res: FxRates = serde_json::from_str(&body)?;

        Ok(res)
    }

    // Simple vector return type API call.
    async fn vec_api_call(&self, path: &str) -> Result<Vec<String>> {
        let url = self.build_url(path)?;
        let body = self.client.get(url).send().await?.text().await?;
        let v: Vec<String> = serde_json::from_str(&body)?;

        Ok(v)
    }

    // Build a URL from the Public API URL plus given path.
    fn build_url(&self, path: &str) -> Result<Url> {
        let s = format!("{}/{}", Self::URL, path);
        let url = Url::parse(&s)?;

        Ok(url)
    }
}

impl Default for Public {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

/// Returned by GetOrderBook.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderBook {
    pub buy_orders: Vec<OrderType>,
    pub sell_orders: Vec<OrderType>,
    created_timestamp_utc: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderType {
    pub order_type: String,
    pub price: f32,
    pub volume: f32,
}

/// Returned by GetMarketSummary
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSummary {
    pub created_timestamp_utc: String,
    pub current_highest_bid_price: f32,
    pub current_lowest_offer_price: f32,
    pub day_avg_price: f32,
    pub day_highest_price: f32,
    pub day_lowest_price: f32,
    pub day_volume_xbt: f32,
    pub day_volume_xbt_in_secondary_currrency: f32,
    pub last_price: f32,
    pub primary_currency_code: String,
    pub secondary_currency_code: String,
}

impl Display for MarketSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match serde_json::to_string_pretty(self) {
            Ok(s) => s,
            Err(_) => serde_json::to_string(self).expect("failed to deserialize market summary"),
        };
        write!(f, "{}", s)
    }
}

/// Returned by GetAllOrders
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orders {
    buy_orders: Vec<OrderGuid>,
    sell_orders: Vec<OrderGuid>,
    created_timestamp_utc: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderGuid {
    guid: String,
    price: f32,
    volume: f32,
}

/// Returned by GetTradeHistorySummary
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TradeHistorySummary {
    history_summary_items: Vec<HistorySummary>,
    number_of_hours_in_the_past_to_retrieve: usize,
    created_timestamp_utc: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HistorySummary {
    start_timestamp_utc: String,
    end_timestamp_utc: String,
    primary_currency_volume: f32,
    secondary_currency_volume: f32,
    opening_secondary_currency_price: f32,
    closing_secondary_currency_price: f32,
    highest_secondary_currency_price: f32,
    lowest_secondary_currency_price: f32,
    average_secondary_currency_price: f32,
    number_of_trades: usize,
}

/// Returned by GetRecentTrades
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecentTrades {
    trades: Vec<Trade>,
    created_timestamp_utc: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trade {
    primary_currency_amount: f32,
    secondary_currency_trade_price: f32,
    trade_timestamp_utc: String,
}

/// Returned by GetFxRates
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FxRates(Vec<Rate>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rate {
    currency_code_a: String,
    currency_code_b: String,
    rate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[tokio::test]
    async fn get_valid_primary_currency_codes_contains_xbt() {
        let api = Public::default();
        let v = api
            .get_valid_primary_currency_codes()
            .await
            .expect("API call failed");

        let want = "Xbt".to_string();
        assert_that(&v).contains(&want);
    }

    #[tokio::test]
    async fn get_valid_secondary_currency_codes_contains_aud() {
        let api = Public::default();
        let v = api
            .get_valid_secondary_currency_codes()
            .await
            .expect("API call failed");

        let want = "Aud".to_string();
        assert_that(&v).contains(&want);
    }

    #[tokio::test]
    async fn get_valid_limit_order_types_contains_bid_and_offer() {
        let api = Public::default();
        let got = api
            .get_valid_limit_order_types()
            .await
            .expect("API call failed");

        let want = vec!["LimitBid", "LimitOffer"];

        for item in want.iter() {
            let want = item.to_string();
            assert_that(&got).contains(&want);
        }
    }

    #[tokio::test]
    async fn get_valid_market_order_types_contains_bid_and_offer() {
        let api = Public::default();
        let got = api
            .get_valid_market_order_types()
            .await
            .expect("API call failed");

        let want = vec!["MarketBid", "MarketOffer"];

        for item in want.iter() {
            let want = item.to_string();
            assert_that(&got).contains(&want);
        }
    }

    #[tokio::test]
    async fn get_valid_order_types_contains_limit_market_bid_offer() {
        let api = Public::default();
        let got = api.get_valid_order_types().await.expect("API call failed");
        let want = vec!["LimitBid", "LimitOffer", "MarketBid", "MarketOffer"];

        for item in want.iter() {
            let want = item.to_string();
            assert_that(&got).contains(&want);
        }
    }

    #[tokio::test]
    async fn get_valid_transaction_types_contains_limit_market_bid_offer() {
        let api = Public::default();
        let got = api
            .get_valid_transaction_types()
            .await
            .expect("API call failed");

        let want = vec![
            "AccountFee",
            "Brokerage",
            "Deposit",
            "DepositFee",
            "GST",
            "ReferralCommission",
            "StatementFee",
            "Trade",
            "Withdrawal",
            "WithdrawalFee",
        ];

        // TODO: Write this helper function or get spectral to work with strings
        // assert_that(&got).contains_all_of(&want);
        // assert_got_contains_all_want(&got, &want);

        for item in want.iter() {
            let want = item.to_string();
            assert_that(&got).contains(&want);
        }
    }

    #[tokio::test]
    async fn can_get_market_summary_xbt_aud() {
        let api = Public::default();
        let _ = api
            .get_market_summary("Xbt", "Aud")
            .await
            .expect("API call failed");
    }

    #[tokio::test]
    async fn can_get_order_book_xbt_aud() {
        let api = Public::default();
        let _ = api
            .get_order_book("Xbt", "Aud")
            .await
            .expect("API call failed");
    }

    #[tokio::test]
    async fn can_get_trade_history_summary_xbt_aud() {
        let api = Public::default();
        let _ = api
            .get_trade_history_summary("Xbt", "Aud", 1)
            .await
            .expect("API call failed");
    }

    #[tokio::test]
    async fn can_get_recent_trades_xbt_aud() {
        let api = Public::default();
        let _ = api
            .get_recent_trades("Xbt", "Aud", 10)
            .await
            .expect("API call failed");
    }

    #[tokio::test]
    async fn can_get_fx_rates() {
        let api = Public::default();
        let _ = api.get_fx_rates().await.expect("API call failed");
    }
}

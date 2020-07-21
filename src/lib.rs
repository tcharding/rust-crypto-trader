pub mod trace;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
// use tracing::debug;
use url::{ParseError, Url};

const IR: &str = "https://api.independentreserve.com";

fn build_public_url(path: &str) -> Result<Url, ParseError> {
    let s = format!("{}/Public/{}", IR, path);
    let base = Url::parse(&s).expect("hardcoded URL is known to be valid");
    Ok(base)
}

pub async fn get_valid_primary_currency_codes(client: Client) -> Result<Vec<String>> {
    let url = build_public_url("GetValidPrimaryCurrencyCodes")?;
    let body = client.get(url).send().await?.text().await?;
    let v: Vec<String> = serde_json::from_str(&body)?;

    Ok(v)
}

pub async fn get_valid_secondary_currency_codes(client: Client) -> Result<Vec<String>> {
    let url = build_public_url("GetValidSecondaryCurrencyCodes")?;

    let body = client.get(url).send().await?.text().await?;
    let v: Vec<String> = serde_json::from_str(&body)?;

    Ok(v)
}

pub async fn get_market_summary(client: Client, base: &str, quote: &str) -> Result<MarketSummary> {
    let url = build_public_url("GetMarketSummary")?;
    let url = Url::parse_with_params(url.as_str(), &[
        ("primaryCurrencyCode", base),
        ("secondaryCurrencyCode", quote),
    ])?;

    let body = client.get(url).send().await?.text().await?;
    let res: MarketSummary = serde_json::from_str(&body)?;

    Ok(res)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn build_public_url_works() {
        let want = format!("{}/Public/foo", IR);
        let got = build_public_url("foo")
            .expect("failed to parse url")
            .to_string();

        assert_that(&got).is_equal_to(&want);
    }
}

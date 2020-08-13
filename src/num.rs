//! Utility functions for working with `Decimal`.
use rust_decimal::Decimal;

/// Decimal places to use for displaying AUD.
const AUD_DP: u32 = 2;

/// Decimal places to use for displaying BTC.
const BTC_DP: u32 = 8;

/// Decimal places to use for displaying a percent.
const PERCENT_DP: u32 = 4;

pub fn to_percent_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(PERCENT_DP))
}

pub fn to_aud_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(AUD_DP))
}

pub fn to_btc_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(BTC_DP))
}

pub fn mid_market_price(bid: &Decimal, ask: &Decimal) -> Decimal {
    (bid + ask) / Decimal::from(2)
}

/// Calculate the spread.
/// Return spread as a raw value and as a percentage of the mid market rate.
pub fn spread_percent(buy: &Decimal, sell: &Decimal) -> (Decimal, Decimal) {
    let price = mid_market_price(buy, sell);
    let spread = buy - sell;
    let spread = spread.abs(); // Maker/taker buy/sells are inverted.
    let percent = spread / price;

    (spread, percent)
}

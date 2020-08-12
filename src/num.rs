//! Utility functions for working with `Decimal`.
use rust_decimal::Decimal;

/// Decimal places to use for displaying AUD.
const AUD_DP: u32 = 2;

/// Decimal places to use for displaying BTC.
const BTC_DP: u32 = 8;

/// Decimal places to use for displaying percentages.
const PERCENTAGE_DP: u32 = 4;

pub fn to_percentage_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(PERCENTAGE_DP))
}

pub fn to_aud_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(AUD_DP))
}

pub fn to_btc_string(x: &Decimal) -> String {
    format!("{}", x.round_dp(BTC_DP))
}

//! This module wraps Decimal. We can give semantic meaning to price and volume
//! here because we are within the `market` module that specifically uses only
//! BTC/AUD. Therefore within this module the following invariants hold:
//! 1. Price is always a quantity in Australian dollars.
//! 2. Volume is always a quantity in bitcoin.
use rust_decimal::Decimal;
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

/// Decimal places to use for displaying AUD.
const AUD_DP: u32 = 2;
/// Decimal places to use for displaying BTC.
const BTC_DP: u32 = 8;
/// Decimal places to use for displaying percentages.
const PERCENTAGE_DP: u32 = 4;

/// Price type so we fully utilize the benefit of static typing.
#[derive(Clone, Copy, Debug, Eq)]
pub struct Price(Decimal);

impl Price {
    pub fn to_percentage(&self) -> String {
        format!("{}", self.0.round_dp(PERCENTAGE_DP))
    }
    pub fn to_dollars(&self) -> String {
        format!("{}", self.0.round_dp(AUD_DP))
    }

    pub fn min_value() -> Self {
        Self(Decimal::min_value())
    }

    pub fn max_value() -> Self {
        Self(Decimal::max_value())
    }
}

impl From<Decimal> for Price {
    fn from(x: Decimal) -> Self {
        Self(x)
    }
}

// TODO: Use checked_ versions?

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let v = self.0 - rhs.0;
        Self(v)
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let v = self.0 + rhs.0;
        Self(v)
    }
}

impl Div for Price {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let v = self.0 / rhs.0;
        Self(v)
    }
}

impl Div<i64> for Price {
    type Output = Self;

    fn div(self, rhs: i64) -> Self {
        let v = self.0 / Decimal::new(rhs, 0);
        Self(v)
    }
}

impl Mul<i64> for Price {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        let v = self.0 * Decimal::new(rhs, 0);
        Self(v)
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Price) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Price) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Price) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.round_dp(AUD_DP))
    }
}

/// Volume type so we fully utilize the benefit of static typing.
#[derive(Clone, Copy, Debug)]
pub struct Volume(Decimal);

impl From<Decimal> for Volume {
    fn from(x: Decimal) -> Self {
        Self(x)
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.round_dp(BTC_DP))
    }
}

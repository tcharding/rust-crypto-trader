use crate::market::api;
use anyhow::{bail, Result};
use num_traits::identities::Zero;
use rust_decimal::Decimal;
use std::{convert::TryFrom, fmt};
use tracing::warn;

#[derive(Clone, Debug)]
pub struct OrderBook {
    /// Sorted list of bids, highest bid first (descending order).
    pub buys: Vec<Order>,
    /// Sorted list of offers, lowest ask first (ascending order).
    pub sells: Vec<Order>,
}

impl OrderBook {
    /// Get the spread if we were to fill a buy and sell order of `volume`.
    pub fn spread_to_fill(&self, volume: Decimal) -> Result<(Decimal, Decimal)> {
        let buy_price = self.price_to_fill_buy_order(volume)?;
        let sell_price = self.price_to_fill_sell_order(volume)?;
        Ok((sell_price, buy_price))
    }

    /// The price if we were to fill a market buy order of `volume`.
    pub fn price_to_fill_buy_order(&self, volume: Decimal) -> Result<Decimal> {
        self.price_to_fill(volume, Position::Buy)
    }

    /// The price if we were to fill a market sell order of `volume`.
    pub fn price_to_fill_sell_order(&self, volume: Decimal) -> Result<Decimal> {
        self.price_to_fill(volume, Position::Sell)
    }

    fn price_to_fill(&self, volume: Decimal, pos: Position) -> Result<Decimal> {
        // Market order matches against the bid/ask e.g., a market buy order
        // matches against an offer (sell).
        let v = match pos {
            Position::Buy => &self.sells,
            Position::Sell => &self.buys,
        };

        let mut still_to_fill = volume;
        let mut total_spend = Decimal::zero();

        for order in v.iter() {
            if still_to_fill > order.volume {
                still_to_fill -= order.volume;
                total_spend += order.volume * order.price;
            } else {
                let partial = still_to_fill;
                still_to_fill = Decimal::zero();
                total_spend += partial * order.price;
            }

            if still_to_fill.is_zero() {
                break;
            }
        }

        if still_to_fill > Decimal::zero() {
            bail!("failed to fill {} order", pos);
        }

        let price = total_spend / volume;
        Ok(price)
    }
}

impl From<api::OrderBook> for OrderBook {
    fn from(orderbook: api::OrderBook) -> Self {
        let mut buys = Vec::with_capacity(orderbook.buy_orders.len());
        for order in orderbook.buy_orders.iter() {
            if let Ok(o) = Order::try_from(order) {
                if o.position == Position::Buy {
                    buys.push(o);
                } else {
                    warn!("non-buy order in buys list");
                }
            }
        }
        buys.sort_unstable_by(|a: &Order, b: &Order| a.price.cmp(&b.price).reverse());

        let mut sells = Vec::with_capacity(orderbook.sell_orders.len());
        for order in orderbook.sell_orders.iter() {
            if let Ok(o) = Order::try_from(order) {
                if o.position == Position::Sell {
                    sells.push(o);
                } else {
                    warn!("non-sell order in sells list");
                }
            }
        }
        sells.sort_unstable_by(|a: &Order, b: &Order| a.price.cmp(&b.price));

        OrderBook { buys, sells }
    }
}

/// Limit order.
#[derive(Clone, Copy, Debug)]
pub struct Order {
    position: Position,
    price: Decimal,
    volume: Decimal,
}

impl TryFrom<&api::PublicOrder> for Order {
    type Error = NullValue;

    fn try_from(order: &api::PublicOrder) -> Result<Self, Self::Error> {
        let price = order.price.ok_or_else(|| NullValue)?;
        let volume = order.volume.ok_or_else(|| NullValue)?;

        Ok(Order {
            position: order.order_type.into(),
            price,
            volume,
        })
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("API returned a null value")]
pub struct NullValue;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    Buy,
    Sell,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::Buy => "buy",
            Position::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

impl From<api::OrderType> for Position {
    fn from(pos: api::OrderType) -> Self {
        match pos {
            api::OrderType::Buy => Position::Buy,
            api::OrderType::Sell => Position::Sell,
        }
    }
}

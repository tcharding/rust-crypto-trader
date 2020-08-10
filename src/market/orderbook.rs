use crate::market;
use std::convert::TryFrom;
use tracing::debug;

#[derive(Clone, Debug)]
pub struct OrderBook {
    /// Sorted list of bids, highest bid first (descending order).
    pub buys: Vec<Order>,
    /// Sorted list of offers, lowest ask first (ascending order).
    pub sells: Vec<Order>,
}

#[derive(Clone, Copy, Debug)]
pub struct Order {
    position: Position,
    price: Num,
    volume: Num,
}

#[derive(Clone, Copy, Debug)]
enum Position {
    Buy,
    Sell,
}

impl OrderBook {
    /// Returns the current best bid/offer spread.
    pub fn bid_offer_spread(&self) -> String {
        if self.buys.is_empty() || self.sells.is_empty() {
            return "no bid/ask data".to_string();
        }

        let highest_bid = self.buys.first().unwrap().price;
        let lowest_offer = self.sells.first().unwrap().price;

        let spread = lowest_offer - highest_bid;

        format!("{}/{} {}", highest_bid, lowest_offer, spread.to_string())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<market::OrderBook> for OrderBook {
    fn from(orderbook: market::OrderBook) -> Self {
        let mut buys = Vec::with_capacity(orderbook.buy_orders.len());
        for order in orderbook.buy_orders.iter() {
            debug_assert!(order.order_type == "LimitBid");
            match Order::try_from(order) {
                Ok(o) => {
                    println!("{} {}", order.price, o.price);
                    buys.push(o);
                }
                Err(e) => debug!("failed to parse order: {}", e),
            }
        }
        buys.sort_unstable_by(|a, b| a.price.cmp(&b.price).reverse());

        let mut sells = Vec::with_capacity(orderbook.sell_orders.len());
        for order in orderbook.sell_orders.iter() {
            debug_assert!(order.order_type == "LimitOffer");
            match Order::try_from(order) {
                Ok(o) => sells.push(o),
                Err(e) => debug!("failed to parse order: {}", e),
            }
        }

        sells.sort_unstable_by(|a, b| a.price.cmp(&b.price));

        OrderBook { buys, sells }
    }
}

impl TryFrom<&market::OrderType> for Order {
    type Error = ApiMalformedData;

    fn try_from(order: &market::OrderType) -> Result<Self, Self::Error> {
        let position = match order.order_type.as_ref() {
            "LimitBid" => Position::Buy,
            "LimitOffer" => Position::Sell,
            other => {
                return Err(ApiMalformedData(other.to_string()));
            }
        };
        let price = Num::from(order.price);
        let volume = Num::from(order.volume);

        Ok(Order {
            position,
            price,
            volume,
        })
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[error("Malformed data returned by API call: {0}")]
pub struct ApiMalformedData(String);

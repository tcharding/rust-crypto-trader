use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Returned by GetOrderBook.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderBook {
    buy_orders: Vec<OrderType>,
    sell_orders: Vec<OrderType>,
    created_timestamp_utc: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct OrderType {
    order_type: String,
    price: f32,
    volume: f32,
}

impl OrderBook {
    /// Returns the current best bid/offer spread.
    pub fn bid_offer_spread(&mut self) -> String {
        self.sort();

        let highest_bid = self.buy_orders.first().unwrap().price;
        let lowest_offer = self.sell_orders.first().unwrap().price;

        let spread = lowest_offer - highest_bid;
        let spread = truncate_to_cents(spread);

        format!("{}/{} {}", highest_bid, lowest_offer, spread)
    }

    /// The price to fill an order of `volume` from the current limit orders.
    pub fn price_to_fill(&self, _order_type: &str, _volume: f32) -> f32 {
        0.0
    }

    /// Sort the buy and sell orders.
    /// bids: descending order (highest bid fist)
    /// offers: ascending order (lowest offer first)
    pub fn sort(&mut self) {
        self.sell_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));

        self.buy_orders
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
        self.buy_orders.reverse();
    }
}

fn truncate_to_cents(x: f32) -> f32 {
    truncate_to_decimal_places(x, 2)
}

// TODO: Write a custom float library? Find some library to use?
/// Truncate x to n decimal places.
fn truncate_to_decimal_places(x: f32, n: usize) -> f32 {
    let mut mul = x;
    for _ in 0..n {
        mul *= 10.0;
    }

    let mut mul = mul.trunc();
    for _ in 0..n {
        mul /= 10.0;
    }
    mul
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    fn buy_order(price: f32, volume: f32) -> OrderType {
        OrderType {
            order_type: "buy".to_string(),
            price,
            volume,
        }
    }

    #[test]
    fn truncating_works() {
        let x = 1.2345_f32;
        let want = 1.23_f32;
        let got = truncate_to_decimal_places(x, 2);
        assert!(approx_eq!(f32, got, want))
    }

    #[test]
    fn price_to_fill_order() {
        let mut buys = vec![];
        buys.push(buy_order(1500.00, 0.9));
        buys.push(buy_order(1550.00, 1.0));

        let orders = OrderBook {
            buy_orders: buys,
            sell_orders: vec![],
            created_timestamp_utc: "nil".to_string(),
            primary_currency_code: "nil".to_string(),
            secondary_currency_code: "nil".to_string(),
        };

        let want = 1545.0;
        let got = orders.price_to_fill("buy", 1.0);
        assert!(approx_eq!(f32, got, want))
    }
}

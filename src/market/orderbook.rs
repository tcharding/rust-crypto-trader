use crate::market::api;
use rust_decimal::Decimal;

#[derive(Clone, Debug)]
pub struct OrderBook {
    /// Sorted list of bids, highest bid first (descending order).
    pub buys: Vec<Order>,
    /// Sorted list of offers, lowest ask first (ascending order).
    pub sells: Vec<Order>,
}

impl OrderBook {
    /// Returns the current best bid/offer spread.
    pub fn bid_ask_spread(&self) -> Option<(Decimal, Decimal)> {
        if self.buys.is_empty() || self.sells.is_empty() {
            return None;
        }

        let highest_bid = self.buys.first().unwrap();
        let lowest_ask = self.sells.first().unwrap();

        let bid = highest_bid.price;
        let ask = lowest_ask.price;

        let mmr = (bid + ask) / Decimal::from(2);
        let spread = ask - bid;
        let percentage = spread / mmr;

        Some((spread, percentage))
    }
}

impl From<api::OrderBook> for OrderBook {
    fn from(orderbook: api::OrderBook) -> Self {
        let mut buys = Vec::with_capacity(orderbook.buy_orders.len());
        for order in orderbook.buy_orders.iter() {
            buys.push(order.into());
        }
        //        buys.sort_unstable_by(|a, b| a.price.cmp(&b.price).reverse());

        let mut sells = Vec::with_capacity(orderbook.sell_orders.len());
        for order in orderbook.sell_orders.iter() {
            sells.push(order.into());
        }
        //        sells.sort_unstable_by(|a, b| a.price.cmp(&b.price));

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

impl From<&api::PublicOrder> for Order {
    fn from(order: &api::PublicOrder) -> Self {
        Order {
            position: order.order_type.into(),
            price: order.price,
            volume: order.volume,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Position {
    Buy,
    Sell,
}

impl From<api::OrderType> for Position {
    fn from(pos: api::OrderType) -> Self {
        match pos {
            api::OrderType::Buy => Position::Buy,
            api::OrderType::Sell => Position::Sell,
        }
    }
}

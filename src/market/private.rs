use anyhow::{bail, Result};
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const PAGE_SIZE: usize = 25;

// GetOpenOrders
// GetClosedOrders
// GetClosedFilledOrders
// GetOrderDetails
// GetAccounts
// GetTransactions
// GetDigitalCurrencyDepositAddress
// GetDigitalCurrencyDepositAddresses
// SynchDigitalCurrencyDepositAddressWithBlockchain
// GetTrades
// GetBrokerageFees
// PlaceLimitOrder
// PlaceMarketOrder
// CancelOrder
// GetDigitalCurrencyWithdrawal
// WithdrawDigitalCurrency
// RequestFiatWithdrawal

/// Implements the private methods for Inedependent Reserve crypto exchange API.
#[derive(Debug)]
pub struct Private {
    client: Client,
    keys: Keys,
    nonce: u64,
}

#[derive(Debug)]
struct Keys {
    /// API key with read-only access.
    read: Key,
}

#[derive(Debug)]
struct Key {
    key: String,
    secret: String,
}

impl Private {
    /// Private API URL
    const URL: &'static str = "https://api.independentreserve.com/Private";

    pub fn new(read_key: impl ToString, read_secret: impl ToString) -> Self {
        Self {
            client: Client::new(),
            keys: Keys {
                read: Key {
                    key: read_key.to_string(),
                    secret: read_secret.to_string(),
                },
            },
            nonce: nonce(),
        }
    }

    /// API call: GetOpenOrders
    pub async fn get_open_orders(
        &mut self,
        base: &str,
        quote: &str,
        page_index: usize,
    ) -> Result<Orders> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetOpenOrders")?;
        let body = self.orders_body(url.clone(), nonce, base, quote, page_index);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let orders: Orders = serde_json::from_str(&body)?;

        Ok(orders)
    }

    /// API call: GetClosedOrders
    pub async fn get_closed_orders(
        &mut self,
        base: &str,
        quote: &str,
        page_index: usize,
    ) -> Result<Orders> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetClosedOrders")?;
        let body = self.orders_body(url.clone(), nonce, base, quote, page_index);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let orders: Orders = serde_json::from_str(&body)?;

        Ok(orders)
    }

    // Build a URL from the Public API URL plus given path.
    fn build_url(&self, path: &str) -> Result<Url> {
        let s = format!("{}/{}", Self::URL, path);
        let url = Url::parse(&s)?;

        Ok(url)
    }

    fn orders_body(
        &self,
        url: Url,
        nonce: u64,
        base: &str,
        quote: &str,
        page_index: usize,
    ) -> OrdersBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!("{},apiKey={},nonce={},primaryCurrencyCode={},secondaryCurrencyCode={},pageIndex={},pageSize={}", url, api_key, nonce, base, quote, page_index, PAGE_SIZE);
        let signature = self.sign_read_only(&msg);

        OrdersBody {
            api_key,
            nonce,
            signature,
            primary_currency_code: base.to_string(),
            secondary_currency_code: quote.to_string(),
            page_index,
            page_size: 25,
        }
    }

    // Signs a message with the read only API secret key.
    fn sign_read_only(&self, msg: &str) -> String {
        sign(msg, &self.keys.read.secret)
    }

    fn inc_nonce(&mut self) -> u64 {
        let nonce = self.nonce;
        self.nonce += 1;
        nonce
    }
}

fn nonce() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_epoch.as_secs()
}

type HmacSha256 = Hmac<Sha256>;

// Returns hex representation of signed message.
fn sign(msg: &str, key: &str) -> String {
    let mut mac = HmacSha256::new_varkey(key.as_bytes()).expect("HMAC can take key of any size");

    mac.update(msg.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    hex::encode(code_bytes)
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "primaryCurrencyCode":"Xbt",
//     "secondaryCurrencyCode":"Usd",
//     "pageIndex":1,
//     "pageSize":25
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersBody {
    signature: String,
    api_key: String,
    nonce: u64,
    primary_currency_code: String,
    secondary_currency_code: String,
    page_index: usize,
    page_size: usize,
}

/// Returned by GetOpenOrders, GetClosedOrders, GetClosedFilledOrders
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orders {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
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

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "orderGuid":"c7347e4c-b865-4c94-8f74-d934d4b0b177"
// }
/// Returned by GetOrderDetails
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderDetails {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: f32,
    volume_filled: f32,
    price: f32,
    avg_price: f32,
    reserved_amount: f32,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}
// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
// }
/// Returned by GetAccounts
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Accounts(Vec<Account>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    account_guid: String,
    account_status: String,
    available_balance: f32,
    currency_code: String,
    total_balance: f32,
}

// {
// "apiKey":"{api-key}",
// "nonce":{nonce},
// "signature":"{signature}",
// "accountGuid":"49994921-60ec-411e-8a78-d0eba078d5e9",
// "fromTimestampUtc":"2014-08-01T09:00:00Z",
// "toTimestampUtc":null,
// "txTypes":["Brokerage","Trade"]
// "pageIndex":1,
// "pageSize":25
// },
/// Returned by GetTransactions
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transactions {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    balance: f32,
    bitcoin_transaction_id: String,
    bitcoin_transaction_output_index: String,
    ethereum_transaction_id: String,
    comment: String,
    created_timestamp_utc: String,
    credit: String,
    currency_code: String,
    debit: f32,
    settle_timestamp_utc: String,
    status: String,
    #[serde(rename = "type")]
    type_: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "primaryCurrencyCode":"Xbt",
//     "pageIndex": 1,
//     "pageSize": 10
// }
/// Returned by GetDigitalCurrencyDepositAddresses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigitalCurrencyDepositAddresses {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<DigitalCurrencyDepositAddress>,
}

// { SynchDigitalCurrencyDepositAddressWithBlockchain
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "depositAddress":"12a7FbBzSGvJd36wNesAxAksLXMWm4oLUJ",
//     "primaryCurrencyCode":"Bch"
// }

// { GetDigitalCurrencyDepositAddress
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "primaryCurrencyCode":"Xbt"
// }
/// Returned by GetDigitalCurrencyDepositAddress,
/// SynchDigitalCurrencyDepositAddressWithBlockchain
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigitalCurrencyDepositAddress {
    deposit_address: String,
    last_checked_timestamp_utc: String,
    next_update_timestamp_utc: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "transactionGuid":"2a93732f-3f40-4685-b3bc-ff3ec326090d",
// }
/// Returned by GetDigitalCurrencyWithdrawal
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDigitalCurrencyWithdrawal {
    transaction_guid: String,
    primary_currency_code: String,
    created_timestamp_utc: String,
    amount: Amount,
    destination: Destination,
    status: String,
    transaction: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "pageIndex":1,
//     "pageSize":5
// }
/// Returned by GetTrades
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trades {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<Trade>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Trade {
    trade_guid: String,
    trade_timestamp_utc: String,
    order_guid: String,
    order_type: String,
    order_timestamp_utc: String,
    volume_traded: f32,
    price: f32,
    primary_currency_code: String,
    secondary_currency_code: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
// }
/// Returned by GetBrokerageFees
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BrokerageFees(Vec<Fees>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fees {
    currency_code: String,
    fee: f32,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "primaryCurrencyCode":"Xbt",
//     "secondaryCurrencyCode":"Usd",
//     "orderType": "LimitBid",
//     "price": 485.76,
//     "volume": 0.358
// }
// {
/// Returned by PlaceLimitOrder
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceLimitOrder {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: f32,
    volume_filled: f32,
    price: f32,
    reserved_amount: f32,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

/// Returned by PlaceMarketOrder
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceMarketOrder {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: f32,
    volume_filled: f32,
    reserved_amount: f32,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

/// Returned by CancelOrder
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelOrder {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: f32,
    volume_filled: f32,
    price: f32,
    reserved_amount: f32,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "amount":0.123,
//     "withdrawalAddress":"1BP2wi6UxQwG3oDuDj2V2Rvgu6PMJnJu61",
//     "comment":"",
//     "primaryCurrencyCode":"Bch"
//     "destinationTag":"123456"
// }
/// Returned by WithdrawDigitalCurrency
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigitalCurrencyWithdrawal {
    transaction_guid: String,
    primary_currency_code: String,
    created_timestamp_utc: String,
    amount: Amount,
    destination: Destination,
    status: String,
    transaction: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Amount {
    total: f32,
    fee: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    address: String,
    tag: String,
}

// {
//     "apiKey":"{api-key}",
//     "nonce":{nonce},
//     "signature":"{signature}",
//     "secondaryCurrencyCode":"{secondaryCurrencyCode}",
//     "withdrawalAmount":"{withdrawalAmount}",
//     "withdrawalBankAccountName":"{withdrawalBankAccountName}",
//     "comment":"{comment}"
// }
/// Returned by RequestFiatwithdrawal
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestFiatwithdrawal {
    account_guid: String,
    created_timestamp_utc: String,
    fiat_withdrawal_request_guid: String,
    status: String,
    total_withdrawal_amonut: f32,
    fee_amount: f32,
    currency: String,
}

use anyhow::{bail, Result};
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

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

// const READER_API_KEY: &str = "b2f7707a-4b1c-4880-b4c4-036d81f3de59";
const READER_API_SECRET: &[u8; 32] = b"get this from the file system";

/// Implements the private methods for Inedependent Reserve crypto exchange API.
pub struct Private {
    client: Client,
}

impl Private {
    /// Private API URL
    const URL: &'static str = "https://api.independentreserve.com/Private";

    // HTTP POST https://api.independentreserve.com/Private/GetOpenOrders
    // {
    //     "apiKey":"{api-key}",
    //     "nonce":{nonce},
    //     "signature":"{signature}",
    //     "primaryCurrencyCode":"Xbt",
    //     "secondaryCurrencyCode":"Usd",
    //     "pageIndex":1,
    //     "pageSize":25
    // }
    pub async fn get_open_orders(&self) -> Result<Orders> {
        let url = self.build_url("GetOpenOrders")?;
        let base = "Xbt".to_string();
        let quote = "Aud".to_string();
        let page_index = 1;
        let page_size = 25;
        let api_key = "b2f7707a-4b1c-4880-b4c4-036d81f3de59";
        let nonce = nonce();

        let msg = format!("{},apiKey={},nonce={},primaryCurrencyCode={},secondaryCurrencyCode={},pageIndex={},pageSize={}", url, api_key, nonce, base, quote, page_index, page_size);
        let sig = sign(&msg);

        let params = ParamsOrders::new(api_key, nonce, &sig);

        let res = self.client.post(url).json(&params).send().await?;

        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let orders: Orders = serde_json::from_str(&body)?;

        Ok(orders)
    }

    pub async fn get_closed_orders(&self) -> Result<Orders> {
        let url = self.build_url("GetClosedOrders")?;
        let base = "Xbt".to_string();
        let quote = "Aud".to_string();
        let page_index = 1;
        let page_size = 25;
        let api_key = "b2f7707a-4b1c-4880-b4c4-036d81f3de59";
        let nonce = nonce();

        let msg = format!("{},apiKey={},nonce={},primaryCurrencyCode={},secondaryCurrencyCode={},pageIndex={},pageSize={}", url, api_key, nonce, base, quote, page_index, page_size);
        let sig = sign(&msg);

        let params = ParamsOrders::new(api_key, nonce, &sig);

        let res = self.client.post(url).json(&params).send().await?;

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
}

impl Default for Private {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
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
fn sign(msg: &str) -> String {
    let mut mac = HmacSha256::new_varkey(READER_API_SECRET).expect("HMAC can take key of any size");

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
pub struct ParamsOrders {
    api_key: String,
    nonce: u64,
    signature: String,
    primary_currency_code: String,
    secondary_currency_code: String,
    page_index: usize,
    page_size: usize,
}

impl ParamsOrders {
    fn new(key: &str, nonce: u64, sig: &str) -> Self {
        Self {
            api_key: key.to_string(),
            nonce,
            signature: sig.to_string(),
            primary_currency_code: "Xbt".to_string(),
            secondary_currency_code: "Aud".to_string(),
            page_index: 1,
            page_size: 25,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_get_orders() {
        let api = Private::default();
        let _ = api.get_open_orders().await.expect("API call failed");
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        let _ = api.get_closed_orders().await.expect("API call failed");
    }
}

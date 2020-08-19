use anyhow::{bail, Context, Result};
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, StatusCode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use url::Url;

const PAGE_SIZE: usize = 25;

// Independent Reserve Private API methods
//
// Read-only Key:
// GetOpenOrders
// GetClosedOrders
// GetClosedFilledOrders
// GetOrderDetails
// GetAccounts
// GetTransactions
// GetDigitalCurrencyDepositAddress
// GetDigitalCurrencyDepositAddresses
// GetTrades
// GetBrokerageFees
// GetDigitalCurrencyWithdrawal
//
// Admin Key:
// SynchDigitalCurrencyDepositAddressWithBlockchain
// PlaceLimitOrder
// PlaceMarketOrder
// CancelOrder
// WithdrawDigitalCurrency
//
// Full access Key:
// RequestFiatWithdrawal

/// Implements the private methods for Inedependent Reserve crypto exchange API.
#[derive(Clone, Debug)]
pub struct Private {
    client: Client,
    keys: Keys,
    nonce: u64,
}

#[derive(Clone, Debug)]
struct Keys {
    /// API key with read-only access.
    read: Key,
}

#[derive(Clone, Debug)]
struct Key {
    key: String,
    secret: String,
}

impl Private {
    /// Private API URL
    const URL: &'static str = "https://api.independentreserve.com/Private";

    pub fn new(nonce: u64, read_key: impl ToString, read_secret: impl ToString) -> Self {
        Self {
            client: Client::new(),
            keys: Keys {
                read: Key {
                    key: read_key.to_string(),
                    secret: read_secret.to_string(),
                },
            },
            nonce,
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

        let body = res
            .text()
            .await
            .with_context(|| format!("no text: {:?}", body))?;
        let orders: Orders = serde_json::from_str(&body)
            .with_context(|| format!("serde failed for body: {:?}", body))?;

        Ok(orders)
    }

    /// API call: GetClosedFilledOrders
    pub async fn get_closed_filled_orders(
        &mut self,
        base: &str,
        quote: &str,
        page_index: usize,
    ) -> Result<Orders> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetClosedFilledOrders")?;
        let body = self.orders_body(url.clone(), nonce, base, quote, page_index);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let orders: Orders = serde_json::from_str(&body)?;

        Ok(orders)
    }

    /// API call: GetOrderDetails
    pub async fn get_order_details(
        &mut self,
        order_guid: &str, // "c7347e4c-b865-4c94-8f74-d934d4b0b177"
    ) -> Result<OrderDetails> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetOrderDetails")?;
        let body = self.order_guid_body(url.clone(), nonce, order_guid);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let details: OrderDetails = serde_json::from_str(&body)?;

        Ok(details)
    }

    /// API call: GetAccounts
    pub async fn get_accounts(&mut self) -> Result<Accounts> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetAccounts")?;
        let body = self.simple_body(url.clone(), nonce);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let accounts: Accounts = serde_json::from_str(&body)?;

        Ok(accounts)
    }

    /// API call: GetTransactions
    pub async fn get_transactions(
        &mut self,
        _a_ccount_guuid: &str,        // "49994921-60ec-411e-8a78-d0eba078d5e9"
        _f_rom: Option<&str>,         // "2014-08-01T08:00:00Z", ISO 8601 standard
        _t_o: Option<&str>,           // Same format as `from`
        _tx_types: Option<Vec<&str>>, // ["Brokerage","Trade"]
        _page_index: usize,
    ) -> Result<Transactions> {
        // {
        //     "apiKey":"{api-key}",
        //     "nonce":{nonce},
        //     "signature":"{signature}",
        //     "accountGuid":
        //     "fromTimestampUtc":"2014-08-01T09:00:00Z",
        //     "toTimestampUtc":null,
        //     "txTypes":
        //     "pageIndex":1,
        //     "pageSize":25
        // }
        unimplemented!()
    }

    /// API call: GetDigitalCurrencyDepositAddress
    pub async fn get_digital_currency_deposit_address(
        &mut self,
        primary_currency_code: &str, // "Xbt"
    ) -> Result<DigitalCurrencyDepositAddress> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetDigitalCurrencyDepositAddress")?;
        let body = self.currency_body(url.clone(), nonce, primary_currency_code);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let address: DigitalCurrencyDepositAddress = serde_json::from_str(&body)?;

        Ok(address)
    }

    /// API call: GetDigitalCurrencyDepositAddresses
    pub async fn get_digital_currency_deposit_addresses(
        &mut self,
        currency: &str, // "Xbt"
        page_index: usize,
    ) -> Result<DigitalCurrencyDepositAddresses> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetDigitalCurrencyDepositAddresses")?;
        let body = self.currency_page_index_body(url.clone(), nonce, currency, page_index);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let addresses: DigitalCurrencyDepositAddresses = serde_json::from_str(&body)?;

        Ok(addresses)
    }

    /// API call: GetTrades
    pub async fn get_trades(&mut self, page_index: usize) -> Result<Trades> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetTrades")?;
        let body = self.page_index_body(url.clone(), nonce, page_index);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let trades: Trades = serde_json::from_str(&body)?;

        Ok(trades)
    }

    /// API call: GetBrokerageFees
    pub async fn get_brokerage_fees(&mut self) -> Result<BrokerageFees> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetBrokerageFees")?;
        let body = self.simple_body(url.clone(), nonce);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let fees: BrokerageFees = serde_json::from_str(&body)?;

        Ok(fees)
    }

    /// API call: GetDigitalCurrencyWithdrawal
    pub async fn get_digital_currency_withdrawal(
        &mut self,
        tx_guid: &str, // "2a93732f-3f40-4685-b3bc-ff3ec326090d",
    ) -> Result<DigitalCurrencyWithdrawal> {
        let nonce = self.inc_nonce();
        let url = self.build_url("GetDigitalCurrencyWithdrawal")?;
        let body = self.tx_guid_body(url.clone(), nonce, tx_guid);

        let res = self.client.post(url).json(&body).send().await?;
        if res.status() != StatusCode::OK {
            bail!("api call returned status: {}", res.status())
        }

        let body = res.text().await?;
        let withdrawal: DigitalCurrencyWithdrawal = serde_json::from_str(&body)?;

        Ok(withdrawal)
    }

    /// API call: SyncDigitalCurrencyDepositAddressWithBlockchain
    pub async fn sync_digital_currency_deposit_address_with_blockchain(
        &mut self,
        _tx_guuid: &str,
    ) -> Result<DigitalCurrencyDepositAddress> {
        // {
        //     "apiKey":"{api-key}",
        //     "nonce":{nonce},
        //     "signature":"{signature}",
        //     "depositAddress":"12a7FbBzSGvJd36wNesAxAksLXMWm4oLUJ",
        //     "primaryCurrencyCode":"Bch"
        // }
        // let nonce = self.inc_nonce();
        // let url = self.build_url("SyncDigitalCurrencyDepositAddressWithBlockchain")?;
        // let body = self.currency_body(url.clone(), nonce, primary_currency_code);

        // let res = self.client.post(url).json(&body).send().await?;
        // if res.status() != StatusCode::OK {
        //     bail!("api call returned status: {}", res.status())
        // }

        // let body = res.text().await?;
        // let address: DigitalCurrencyDepositAddress = serde_json::from_str(&body)?;

        // Ok(address)
        unimplemented!()
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

    fn simple_body(&self, url: Url, nonce: u64) -> SimpleBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!("{},apiKey={},nonce={}", url, api_key, nonce);
        let signature = self.sign_read_only(&msg);

        SimpleBody {
            api_key,
            nonce,
            signature,
        }
    }

    fn order_guid_body(&self, url: Url, nonce: u64, guid: &str) -> OrderGuidBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!(
            "{},apiKey={},nonce={},orderGuid={}",
            url, api_key, nonce, guid
        );
        let signature = self.sign_read_only(&msg);

        OrderGuidBody {
            api_key,
            nonce,
            order_guid: guid.to_string(),
            signature,
        }
    }

    fn currency_body(&self, url: Url, nonce: u64, currency: &str) -> CurrencyBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!(
            "{},apiKey={},nonce={},primaryCurrencyCode={}",
            url, api_key, nonce, currency,
        );
        let signature = self.sign_read_only(&msg);

        CurrencyBody {
            api_key,
            nonce,
            primary_currency_code: currency.to_string(),
            signature,
        }
    }

    fn tx_guid_body(&self, url: Url, nonce: u64, guid: &str) -> TxGuidBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!(
            "{},apiKey={},nonce={},transactionGuid={}",
            url, api_key, nonce, guid
        );
        let signature = self.sign_read_only(&msg);

        TxGuidBody {
            api_key,
            nonce,
            transaction_guid: guid.to_string(),
            signature,
        }
    }

    fn page_index_body(&self, url: Url, nonce: u64, page_index: usize) -> PageIndexBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!(
            "{},apiKey={},nonce={},pageIndex={},pageSize={}",
            url, api_key, nonce, page_index, PAGE_SIZE
        );
        let signature = self.sign_read_only(&msg);

        PageIndexBody {
            api_key,
            nonce,
            page_index,
            page_size: PAGE_SIZE,
            signature,
        }
    }

    fn currency_page_index_body(
        &self,
        url: Url,
        nonce: u64,
        currency: &str,
        page_index: usize,
    ) -> CurrencyPageIndexBody {
        let api_key = self.keys.read.key.clone();

        let msg = format!(
            "{},apiKey={},nonce={},primaryCurrencyCode={},pageIndex={},pageSize={}",
            url, api_key, nonce, currency, page_index, PAGE_SIZE,
        );
        let signature = self.sign_read_only(&msg);

        CurrencyPageIndexBody {
            api_key,
            nonce,
            primary_currency_code: currency.to_string(),
            page_index,
            page_size: PAGE_SIZE,
            signature,
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

type HmacSha256 = Hmac<Sha256>;

// Returns hex representation of signed message.
fn sign(msg: &str, key: &str) -> String {
    let mut mac = HmacSha256::new_varkey(key.as_bytes()).expect("HMAC can take key of any size");

    mac.update(msg.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    hex::encode(code_bytes)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleBody {
    signature: String,
    api_key: String,
    nonce: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderGuidBody {
    signature: String,
    api_key: String,
    nonce: u64,
    order_guid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyBody {
    signature: String,
    api_key: String,
    nonce: u64,
    primary_currency_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TxGuidBody {
    signature: String,
    api_key: String,
    nonce: u64,
    transaction_guid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageIndexBody {
    signature: String,
    api_key: String,
    nonce: u64,
    page_index: usize,
    page_size: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyPageIndexBody {
    signature: String,
    api_key: String,
    nonce: u64,
    primary_currency_code: String,
    page_index: usize,
    page_size: usize,
}

/// Returned by GetOpenOrders, GetClosedOrders, GetClosedFilledOrders
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orders {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<Order>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    avg_price: Decimal,
    created_timestamp_utc: String,
    fee_percent: Decimal,
    order_guid: String,
    order_type: String,
    outstanding: Decimal,
    price: Option<Decimal>,
    primary_currency_code: String,
    secondary_currency_code: String,
    status: String,
    value: Decimal,
    volume: Decimal,
}

/// Returned by GetOrderDetails
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderDetails {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: Decimal,
    volume_filled: Decimal,
    price: Decimal,
    avg_price: Decimal,
    reserved_amount: Decimal,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

/// Returned by GetAccounts
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Accounts(Vec<Account>);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    account_guid: String,
    account_status: String,
    available_balance: Decimal,
    currency_code: String,
    total_balance: Decimal,
}

/// Returned by GetTransactions
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transactions {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<Transaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    balance: Decimal,
    bitcoin_transaction_id: String,
    bitcoin_transaction_output_index: String,
    ethereum_transaction_id: String,
    comment: String,
    created_timestamp_utc: String,
    credit: String,
    currency_code: String,
    debit: Decimal,
    settle_timestamp_utc: String,
    status: String,
    #[serde(rename = "type")]
    type_: String,
}

/// Returned by GetDigitalCurrencyDepositAddress,
/// SyncDigitalCurrencyDepositAddressWithBlockchain
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigitalCurrencyDepositAddress {
    deposit_address: String,
    last_checked_timestamp_utc: String,
    next_update_timestamp_utc: String,
}

/// Returned by GetDigitalCurrencyDepositAddresses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DigitalCurrencyDepositAddresses {
    total_items: usize,
    page_size: usize,
    total_pages: usize,
    data: Vec<DigitalCurrencyDepositAddress>,
}

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
    volume_traded: Decimal,
    price: Decimal,
    primary_currency_code: String,
    secondary_currency_code: String,
}

/// Returned by GetBrokerageFees
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BrokerageFees(Vec<Fees>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fees {
    currency_code: String,
    fee: Decimal,
}

/// Returned by PlaceLimitOrder
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaceLimitOrder {
    order_guid: String,
    created_timestamp_utc: String,
    #[serde(rename = "type")]
    type_: String,
    volume_ordered: Decimal,
    volume_filled: Decimal,
    price: Decimal,
    reserved_amount: Decimal,
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
    volume_ordered: Decimal,
    volume_filled: Decimal,
    reserved_amount: Decimal,
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
    volume_ordered: Decimal,
    volume_filled: Decimal,
    price: Decimal,
    reserved_amount: Decimal,
    status: String,
    primary_currency_code: String,
    secondary_currency_code: String,
}

/// Returned by WithdrawDigitalCurrency
#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Amount {
    total: Decimal,
    fee: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    address: String,
    tag: String,
}

/// Returned by RequestFiatwithdrawal
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestFiatwithdrawal {
    account_guid: String,
    created_timestamp_utc: String,
    fiat_withdrawal_request_guid: String,
    status: String,
    total_withdrawal_amonut: Decimal,
    fee_amount: Decimal,
    currency: String,
}

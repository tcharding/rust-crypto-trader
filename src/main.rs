#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
use anyhow::Result;
use log::LevelFilter;
use reqwest::Client;
use rust_crypto_trader::{trace, *};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust trading bot - enjoy!");

    trace::init_tracing(LevelFilter::Debug)?;

    let client = Client::new();
    //    let v = get_valid_secondary_currency_codes(client).await?;
    let ms = get_market_summary(client, "Xbt", "Aud").await?;
    println!("{}", ms);

    Ok(())
}

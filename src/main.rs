#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
use anyhow::Result;
use reqwest::Client;
use rust_crypto_trader::*;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust trading bot - enjoy!");

    let client = Client::new();
    let _ = get_valid_primary_currency_codes(client).await?;

    loop {
        println!("looping ...");
        thread::sleep(Duration::from_secs(2));
    }
}

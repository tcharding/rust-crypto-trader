use anyhow::{Context, Result};
use chrono::Utc;
use log::LevelFilter;
use rust_decimal::Decimal;
use std::{fmt, fs::OpenOptions, io::prelude::*, process, time::Duration};
use tracing::error;

use crypto_trader::{config, market::Market, num, trace};

/// Crypto-trader configuration file.
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

/// Bot output file.
const OUT_FILE: &str = "spread-bot.out";

#[tokio::main]
pub async fn main() -> Result<()> {
    let path = directories::UserDirs::new()
        .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
        .expect("failed to construct config path");

    trace::init_tracing(LevelFilter::Trace)?;

    let config =
        config::parse(&path).with_context(|| format!("config file: {}", path.display()))?;

    let mut values = MinMax::default();
    let m = Market::default().with_read_only(config.keys.read);

    let mut counter = 0;
    loop {
        update_values(&m, &mut values).await;

        // We loop every second, so write file every 5 minuets
        if counter == 0 {
            write_to_file(OUT_FILE, &values).await;
            counter = 0;
        }

        tokio::time::delay_for(Duration::from_secs(1)).await;

        counter += 1;
        if counter > 60 * 5 {
            counter = 0;
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MinMax {
    min_spread: Decimal,
    max_spread: Decimal,
    min_percentage: Decimal,
    max_percentage: Decimal,
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "spread min: {} max: {} \t percentage min: {} max: {}",
            self.min_spread, self.max_spread, self.min_percentage, self.max_percentage,
        )
    }
}

impl Default for MinMax {
    fn default() -> Self {
        Self {
            min_spread: Decimal::max_value(),
            min_percentage: Decimal::max_value(),
            max_spread: Decimal::min_value(),
            max_percentage: Decimal::min_value(),
        }
    }
}

/// Get orderbook then calculate and store spread/percentage values.
async fn update_values(m: &Market, v: &mut MinMax) {
    let orderbook = m.order_book().await.expect("failed to get orderbook");

    if let Some((spread, percentage)) = orderbook.bid_ask_spread() {
        if spread < v.min_spread {
            v.min_spread = spread;
        }
        if spread > v.max_spread {
            v.max_spread = spread;
        }

        if percentage < v.min_percentage {
            v.min_percentage = percentage;
        }
        if percentage > v.max_percentage {
            v.max_percentage = percentage;
        }
    }
}

/// Write values to file.
async fn write_to_file(file: &str, v: &MinMax) {
    let when = Utc::now().naive_local();

    let s = format!(
        "{} spread(min/max) %(min/max): {}/{} {}/{}",
        when.format("%Y-%m-%d %H:%M:%S").to_string(),
        num::to_aud_string(&v.min_spread),
        num::to_aud_string(&v.max_spread),
        num::to_percentage_string(&v.min_percentage),
        num::to_percentage_string(&v.max_percentage),
    );

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file)
    {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open file: {}", e);
            process::exit(1); // Does this work with async code?
        }
    };

    if let Err(e) = writeln!(file, "{}", s) {
        error!("Couldn't write to file: {}", e);
    }
}

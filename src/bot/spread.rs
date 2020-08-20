use anyhow::{Context, Result};
use chrono::prelude::*;
use rust_decimal::Decimal;
use std::{fmt, fs::OpenOptions, io::prelude::*, time::Duration};
use tracing::{error, info};

use crate::{config::Key, market::Market, num};

/// Bot output log file.
const LOG_FILE: &str = "spread-bot.log";

const SAMPLE_PERIOD_SECS: u64 = 5; // Get orderbook every X seconds.
const LOG_ENTRY_PERIOD_SECS: u64 = 3600; // Once an hour

/// Entry point for the spread-bot
pub async fn run(read: Key) -> Result<()> {
    let mut values = MinMax::default();
    let m = Market::default().with_read_only(read);

    info!("writing min/max values to {}", LOG_FILE);
    write_to_file(LOG_FILE, &values).await?;

    let mut loop_counter = 0;
    loop {
        update_values(&m, &mut values).await;

        let time_running = loop_counter * SAMPLE_PERIOD_SECS;

        if time_running > LOG_ENTRY_PERIOD_SECS {
            write_to_file(LOG_FILE, &values).await?;

            values = MinMax::default();
            loop_counter = 0;
        } else {
            loop_counter += 1;
        }

        tokio::time::delay_for(Duration::from_secs(SAMPLE_PERIOD_SECS)).await;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MinMax {
    min_spread: Decimal,
    max_spread: Decimal,
    min_percent: Decimal,
    max_percent: Decimal,
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "spread min: {} max: {} \t percent min: {} max: {}",
            self.min_spread, self.max_spread, self.min_percent, self.max_percent,
        )
    }
}

impl Default for MinMax {
    fn default() -> Self {
        Self {
            min_spread: Decimal::max_value(),
            max_spread: Decimal::min_value(),

            min_percent: Decimal::max_value(),
            max_percent: Decimal::min_value(),
        }
    }
}

/// Get orderbook then calculate and store spread/percent values.
async fn update_values(m: &Market, v: &mut MinMax) {
    let orderbook = m.order_book().await.expect("failed to get orderbook");

    let (bid, ask) = match orderbook.spread_to_fill(Decimal::from(1)) {
        Ok(s) => s,
        Err(e) => {
            info!("failed to get spread: {}", e);
            return;
        }
    };

    let (spread, percent) = num::spread_percent(&bid, &ask);

    if spread < v.min_spread {
        v.min_spread = spread;
    }
    if spread > v.max_spread {
        v.max_spread = spread;
    }

    if percent < v.min_percent {
        v.min_percent = percent;
    }
    if percent > v.max_percent {
        v.max_percent = percent;
    }
}

/// Write values to file.
async fn write_to_file(file: &str, v: &MinMax) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file)
        .with_context(|| format!("failed to open/create file: {}", file))?;

    let s = log_entry(v);
    if let Err(e) = writeln!(file, "{}", s) {
        error!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn log_entry(v: &MinMax) -> String {
    let local: DateTime<Local> = Local::now();

    format!(
        "{} spread: $ min/max % min/max: {} / {}    {} / {}",
        local.format("%Y-%m-%d %H:%M:%S").to_string(),
        num::to_aud_string(&v.min_spread),
        num::to_aud_string(&v.max_spread),
        num::to_percent_string(&v.min_percent),
        num::to_percent_string(&v.max_percent),
    )
}

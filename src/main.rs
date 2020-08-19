use anyhow::{Context, Result};
use log::LevelFilter;
use std::{fs, path::Path, process};
use structopt::StructOpt;

use crypto_trader::{
    bot::spread,
    cli::{self, Cmd},
    config, market, trace,
};

/// Crypto-trader configuration files (we pre-pend HOME to these).
const CONFIG_FILE: &str = ".config/crypto-trader/config.toml";

#[tokio::main]
pub async fn main() -> Result<()> {
    let options = cli::Options::from_args();

    let config_path = options.config_file.unwrap_or_else(|| {
        directories::UserDirs::new()
            .map(|d| d.home_dir().to_path_buf().join(CONFIG_FILE))
            .expect("failed to construct config path")
    });

    if options.dump_config {
        dump_config(&config_path)?;
        process::exit(0);
    }

    trace::init_tracing(LevelFilter::Trace)?;

    let config = config::parse(&config_path)
        .with_context(|| format!("config file: {}", config_path.display()))?;
    // tracing::debug!("{:?}", config);

    if options.cmd.is_none() {
        println!("no command supplied, running API tests ...");
        market::test_ir_api(config.ir.read_only).await;
        process::exit(0);
    }

    match options.cmd.unwrap() {
        Cmd::Test => market::test_ir_api(config.ir.read_only).await,
        Cmd::SpreadBot => spread::run(config.ir.read_only).await?,
    }

    Ok(())
}

fn dump_config(path: &Path) -> anyhow::Result<()> {
    let s = fs::read_to_string(path)?;
    println!("Read config file: \n\n{}", s);

    Ok(())
}

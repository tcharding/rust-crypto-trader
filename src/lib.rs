#![warn(
    unused_extern_crates,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::fallible_impl_from,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::print_stdout,
    clippy::dbg_macro
)]
#![forbid(unsafe_code)]

pub mod config;
pub mod market;
pub mod trace;

pub use crate::config::*;

use std::time::{SystemTime, UNIX_EPOCH};

fn nonce() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_epoch.as_secs()
}

use anyhow::Result;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    println!("Rust trading bot - enjoy!");

    loop {
        println!("looping ...");
        thread::sleep(Duration::from_secs(1));
    }
}

#![allow(clippy::or_fun_call)]
#![allow(clippy::match_bool)]

mod config;
mod display;
mod fetch;
mod sources;

use clap::Parser;
use config::Config;
use eyre::Result;
use fetch::Fetch;

fn main() {
    let _ = setup();
    let config = Config::parse();
    let fetch = Fetch::new_with_config(&config);
    println!("{fetch}");
}

fn setup() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Warn)
        .try_init()?;
    Ok(())
}

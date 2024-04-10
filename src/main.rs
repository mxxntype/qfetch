mod colors;
mod config;
mod fetch;
mod ram;
mod user;

use clap::Parser;
use config::Config;
use eyre::Result;
use fetch::Fetch;

fn main() {
    let _ = setup();
    let config = Config::parse();
    let fetch = Fetch::with_config(&config);
    fetch.render();
}

fn setup() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Warn)
        .try_init()?;
    Ok(())
}

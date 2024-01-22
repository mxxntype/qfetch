mod colors;
mod fetch;
mod ram;
mod user;

use eyre::Result;
use fetch::Fetch;

fn main() {
    let _ = setup();
    let fetch = Fetch::new();
    fetch.render();
}

fn setup() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Warn)
        .try_init()?;
    Ok(())
}

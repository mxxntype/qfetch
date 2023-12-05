mod hostname;
mod user;

use eyre::Result;

use crate::{hostname::Hostname, user::User};

fn main() -> Result<()> {
    setup()?;

    let user = User::get();
    let hostname = Hostname::get();

    println!();
    println!("    user: {}", user);
    println!("hostname: {}", hostname);
    println!();

    Ok(())
}

fn setup() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Warn)
        .try_init()?;
    Ok(())
}

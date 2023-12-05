mod colors;
mod ram;
mod user;

use crate::{ram::Ram, user::User};
use eyre::Result;
use owo_colors::OwoColorize;
use sysinfo::{RefreshKind, System, SystemExt};

fn main() -> Result<()> {
    setup()?;

    let system = System::new_with_specifics(RefreshKind::new().with_memory());
    let user = User::get();
    let ram = Ram::new(&system);

    println!();
    println!("\t󰀄  {user}");
    println!(
        "\t󰒋  {}",
        system
            .host_name()
            .unwrap_or_else(|| "Unknown".into())
            .magenta()
    );
    println!(
        "\t󰹻  {}",
        system.name().unwrap_or_else(|| "Unknown".into()).green()
    );
    println!("\t󰍛  {ram}");
    println!("\t   {}", colors::colors());
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

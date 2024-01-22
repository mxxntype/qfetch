use crate::{colors, ram::Ram, user::User};
use owo_colors::OwoColorize;
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Debug)]
pub struct Fetch {
    system: System,
    ram: Ram,
    user: User,
}

impl Fetch {
    pub fn new() -> Self {
        let system = System::new_with_specifics(RefreshKind::new().with_memory());
        let ram = Ram::new(&system);
        Self {
            system,
            ram,
            user: User::get(),
        }
    }

    pub fn render(&self) {
        println!();
        println!("\t󰀄  {}", self.user);
        println!(
            "\t󰒋  {}",
            format!(
                "{} (Up {} hours)",
                self.system.host_name().unwrap_or_else(|| "Unknown".into()),
                self.system.uptime() / 60 / 60
            )
            .magenta()
        );
        println!(
            "\t󰹻  {}",
            format!(
                "{} {}",
                self.system
                    .name()
                    .unwrap_or_else(|| String::from("Unknown")),
                self.system.os_version().unwrap_or_default()
            )
            .green()
        );
        println!("\t󰍛  {}", self.ram);
        println!("\t   {}", colors::colors());
        println!();
    }
}

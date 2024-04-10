use crate::{colors, config::Config, ram::Ram, user::User};
use owo_colors::OwoColorize;
use sysinfo::{RefreshKind, System, SystemExt};

#[derive(Debug)]
pub struct Fetch<'cfg> {
    system: System,
    ram: Ram,
    user: User,
    config: &'cfg Config,
}

impl<'cfg> Fetch<'cfg> {
    pub fn with_config(config: &'cfg Config) -> Self {
        let system = System::new_with_specifics(RefreshKind::new().with_memory());
        let ram = Ram::new(&system);
        Self {
            system,
            ram,
            user: User::get(),
            config,
        }
    }

    pub fn render(&self) {
        let icons = self.config.icons;

        println!();

        // Print the user.
        let prefix = if icons { "󰀄 " } else { "User:" };
        println!("\t{} {}", prefix, self.user);

        // Print the host.
        let prefix = if icons { "󰒋 " } else { "Host:" };
        let hostname = self.system.host_name().unwrap_or("Unknown".into());
        let uptime = self.system.uptime() as f64 / 3600.0;
        let info = format!("{} (Up {:.1}h)", hostname, uptime);
        println!("\t{} {}", prefix, info.magenta());

        // Print the OS.
        let os_name = self.system.name().unwrap_or("Unknown".into());
        let os_version = self.system.os_version().unwrap_or_default();
        let prefix = if icons { "󰹻 " } else { "  OS:" };
        let info = format!("{} {}", os_name, os_version);
        println!("\t{} {}", prefix, info.green());

        // Print the ram.
        let prefix = if icons { "󰍛 " } else { " RAM:" };
        println!("\t{} {}", prefix, self.ram);

        // Print the color swatches.
        println!("\t   {}", colors::colors());

        println!();
    }
}

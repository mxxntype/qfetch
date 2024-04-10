use crate::{colors, config::Config, ram::Ram, user::User};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

/// The fetch itself, represented as a struct.
///
/// This struct holds all the information it needs to display, as well
/// as the [`Config`] options that may be passed via CLI arguments.
#[derive(Debug)]
pub struct Fetch<'cfg> {
    system: System,
    ram: Ram,
    user: User,
    config: &'cfg Config,
}

impl<'cfg> Fetch<'cfg> {
    /// Instantiate a new [`Fetch`].
    ///
    /// An instance created this way will respect options in the provided [`Config`].
    pub fn with_config(config: &'cfg Config) -> Self {
        let refreshes = RefreshKind::new().with_memory().with_disks_list();
        let system = System::new_with_specifics(refreshes);
        let ram = Ram::new(&system);

        Self {
            system,
            ram,
            user: User::get(),
            config,
        }
    }

    /// Render the fetch to `stdout`.
    //
    // TODO: Implement this via the `std::fmt::Display` trait.
    pub fn render(&self) {
        let icons = self.config.icons;

        println!();

        // Print the user.
        let prefix = if icons { "󰀄 " } else { "User:" };
        println!("\t{prefix} {}", self.user);

        // Print the host.
        let prefix = if icons { "󰒋 " } else { "Host:" };
        let hostname = self.system.host_name().unwrap_or("Unknown".into());
        let uptime = self.system.uptime() as f64 / 3600.0;
        let info = format!("{hostname} (Up {uptime:.1}h)");
        println!("\t{prefix} {}", info.magenta());

        // Print the OS.
        let os_name = self.system.name().unwrap_or("Unknown".into());
        let os_version = self.system.os_version().unwrap_or_default();
        let prefix = if icons { "󰹻 " } else { "  OS:" };
        let info = format!("{os_name} {os_version}");
        println!("\t{prefix} {}", info.green());

        // Print the ram.
        let prefix = if icons { "󰍛 " } else { " RAM:" };
        println!("\t{} {}", prefix, self.ram);

        // Print the root disk.
        let root_disk = self
            .system
            .disks()
            .iter()
            .find(|d| d.mount_point() == PathBuf::from("/"));
        if let Some(root_disk) = root_disk {
            let total_gb = root_disk.total_space() / 1024u64.pow(3);
            let fstype = String::from_utf8(root_disk.file_system().to_vec()).unwrap();
            let prefix = if icons { "󰋊 " } else { "Disk:" };
            let info = format!("{total_gb}GB ({fstype})");
            println!("\t{prefix} {}", info.cyan());
        }

        // Print the color swatches.
        println!("\t   {}", colors::colors());

        println!();
    }
}

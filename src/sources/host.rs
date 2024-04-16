use super::FetchSource;
use crate::display::icons;
use sysinfo::{System, SystemExt};

pub struct Host {
    name: String,
    uptime: u64,
}

impl FetchSource for Host {
    #[allow(clippy::cast_precision_loss)]
    fn info(&self) -> String {
        format!("{} (Up {:.1}h)", self.name, self.uptime as f64 / 3600.0)
    }

    fn icon(&self) -> char {
        icons::HOST
    }

    fn text_prefix(&self) -> String {
        "host".into()
    }
}

impl Host {
    pub fn get(system: &System) -> Self {
        let name = system.host_name().unwrap_or("Unknown".into());
        let uptime = system.uptime();
        Self { name, uptime }
    }
}

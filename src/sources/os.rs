use crate::display::icons;

use super::FetchSource;
use sysinfo::{System, SystemExt};

pub struct Os {
    name: String,
    version: String,
}

impl FetchSource for Os {
    fn info(&self) -> String {
        format!("{} {}", self.name, self.version)
    }

    fn icon(&self) -> char {
        icons::OS
    }

    fn text_prefix(&self) -> String {
        "os".into()
    }
}

impl Os {
    pub fn get(system: &System) -> Self {
        let name = system.name().unwrap_or("Unknown".into());
        let version = system.os_version().unwrap_or("Unknown".into());
        Self { name, version }
    }
}

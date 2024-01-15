use owo_colors::OwoColorize;
use std::fmt;
use sysinfo::{System, SystemExt};

pub struct Ram {
    total: u64,
    used: u64,
}

impl Ram {
    pub fn new(system: &System) -> Self {
        Self {
            total: system.total_memory(),
            used: system.used_memory(),
        }
    }
}

impl fmt::Display for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let used_mb = self.used / (1024 * 1024);
        let total_mb = self.total / (1024 * 1024);
        write!(
            f,
            "{}/{} MB",
            used_mb.bold().yellow(),
            total_mb.bold().red()
        )?;
        Ok(())
    }
}

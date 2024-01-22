use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter, Result};
use sysinfo::{System, SystemExt};

#[derive(Debug)]
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

#[allow(clippy::cast_precision_loss)]
impl Display for Ram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        const GB: f64 = 1024u32.pow(3) as f64;
        let used_gb = self.used as f64 / GB;
        let total_gb = self.total as f64 / GB;
        write!(
            f,
            "{:.1}/{:.1} GB",
            used_gb.bold().yellow(),
            total_gb.bold().red()
        )?;
        Ok(())
    }
}

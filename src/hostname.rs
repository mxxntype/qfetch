use nix::unistd::gethostname;
use owo_colors::OwoColorize;
use std::fmt;

pub struct Hostname {
    name: String,
}

impl Hostname {
    pub fn get() -> Self {
        match gethostname() {
            Ok(name) => Self {
                name: name.to_string_lossy().to_string(),
            },
            Err(_) => Self::default(),
        }
    }
}

impl Default for Hostname {
    fn default() -> Self {
        Self {
            name: "host".into(),
        }
    }
}

impl fmt::Display for Hostname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.purple())?;
        Ok(())
    }
}

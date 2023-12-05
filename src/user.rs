use owo_colors::OwoColorize;
use std::{env, fmt};

#[derive(Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn get() -> Self {
        if let Ok(name) = env::var("USER") {
            Self { name }
        } else {
            log::warn!("Could not read $USER");
            Self::default()
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "user".into(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name.as_str() {
            "root" => write!(f, "{}", self.name.red())?,
            _ => write!(f, "{}", self.name.blue())?,
        }
        Ok(())
    }
}

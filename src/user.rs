use owo_colors::OwoColorize;
use std::{env, fmt};

#[derive(Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn get() -> Self {
        env::var("USER").map_or_else(
            |error_msg| {
                log::warn!("Could not read $USER ({error_msg})");
                Self::default()
            },
            |name| Self { name },
        )
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

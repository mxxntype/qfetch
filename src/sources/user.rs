use super::FetchSource;
use crate::display::icons;
use std::{env, fmt};

#[derive(Debug)]
pub struct User(String);

impl FetchSource for User {
    fn info(&self) -> String {
        self.0.clone()
    }

    fn icon(&self) -> char {
        icons::USER
    }

    fn text_prefix(&self) -> String {
        "user".into()
    }
}

impl User {
    pub fn get() -> Self {
        match env::var("USER") {
            Ok(name) => Self(name),
            Err(error) => {
                log::warn!("Failed to get the username: {error}");
                Self::default()
            }
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self("user".into())
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

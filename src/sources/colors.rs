use super::FetchSource;
use crate::display::icons::COLOR;
use owo_colors::OwoColorize;

pub struct Colors(String);

impl FetchSource for Colors {
    fn info(&self) -> String {
        self.0.clone()
    }

    fn icon(&self) -> char {
        ' '
    }

    fn text_prefix(&self) -> String {
        String::new()
    }
}

impl Colors {
    /// Get colors with 7 [`SWATCH`]es (one for each terminal color).
    pub fn get() -> Self {
        Self(format!(
            "{} {} {} {} {} {} {} {}",
            COLOR.bright_black(),
            COLOR.red(),
            COLOR.yellow(),
            COLOR.green(),
            COLOR.cyan(),
            COLOR.blue(),
            COLOR.magenta(),
            COLOR
        ))
    }
}

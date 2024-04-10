use owo_colors::OwoColorize;

const SWATCH: &str = "■";

pub fn colors() -> String {
    format!(
        "{} {} {} {} {} {} {}",
        SWATCH.bright_black(),
        SWATCH.red(),
        SWATCH.yellow(),
        SWATCH.green(),
        SWATCH.cyan(),
        SWATCH.blue(),
        SWATCH.magenta(),
    )
}

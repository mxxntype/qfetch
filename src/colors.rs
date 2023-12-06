use owo_colors::OwoColorize;

pub fn colors() -> String {
    format!(
        "{} {} {} {} {} {} {}",
        "".bright_black(),
        "".red(),
        "".yellow(),
        "".green(),
        "".cyan(),
        "".blue(),
        "".magenta(),
    )
}

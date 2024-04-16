use clap::Parser;

#[derive(Debug, Default, Parser)]
pub struct Config {
    /// Use icons instead of textual prefixes for entries.
    #[arg(short, long)]
    pub icons: bool,
}

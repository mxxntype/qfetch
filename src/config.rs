use clap::Parser;

#[derive(Debug, Default, Parser)]
pub struct Config {
    #[arg(short, long)]
    pub icons: bool,
}

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    zerum::cli::Cli::parse().run()
}

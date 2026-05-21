use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    match zerum::cli::Cli::parse().run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(zerum::cli::EXIT_ERROR)
        }
    }
}

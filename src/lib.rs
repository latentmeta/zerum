//! Zerum — deterministic-first Python code governance.
//!
//! Repository: <https://github.com/latentmeta/zerum>
//!
//! Phase 1 ships native checks ZR001–ZR010, a [`cli::Cli`] entrypoint, and reporters.
//! Use the `zerum` binary (`cargo install zerum`) for day-to-day analysis.

pub mod analyzers;
pub mod checks;
pub mod cli;
pub mod config;
pub mod core;
pub mod discovery;
pub mod integrations;
pub mod llm;
pub mod parser;
pub mod reporters;

use crate::analyzers::DeterministicAnalyzer;
use crate::config::Config;
use crate::core::CheckRegistry;
use crate::discovery::discover_python_files;
use crate::reporters::{ReportKind, render};
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Exit code when issues are found.
pub const EXIT_ISSUES: u8 = 1;
/// Exit code for operational failures (I/O, parse errors on all files, CLI errors).
pub const EXIT_ERROR: u8 = 2;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ReportFormat {
    #[default]
    Human,
    Json,
}

impl From<ReportFormat> for ReportKind {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Human => ReportKind::Human,
            ReportFormat::Json => ReportKind::Json,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "zerum", version, about = "Deterministic code governance for Python")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run deterministic checks on a path (exits 1 when issues are found)
    Check {
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = ReportFormat::Human)]
        format: ReportFormat,
    },
    /// Explain a check by id (e.g. ZR001)
    Explain {
        id: String,
    },
    /// Write a starter zerum.toml in the current directory
    Init,
    /// List built-in deterministic checks
    ListChecks,
}

impl Cli {
    pub fn run(self) -> Result<ExitCode> {
        match self.command {
            Commands::Check {
                path,
                format,
            } => run_check(&path, format),
            Commands::Explain { id } => run_explain(&id),
            Commands::Init => run_init(),
            Commands::ListChecks => run_list_checks(),
        }
    }
}

fn run_check(path: &Path, format: ReportFormat) -> Result<ExitCode> {
    let config = Config::discover(path)?;
    let files = discover_python_files(path)?;
    if files.is_empty() {
        bail!("no Python files found under {}", path.display());
    }
    let analyzer = DeterministicAnalyzer::new();
    let result = analyzer.analyze_paths(&files, &config);
    println!("{}", render(format.into(), &result.issues)?);

    if !result.file_errors.is_empty() && result.issues.is_empty() {
        return Ok(ExitCode::from(EXIT_ERROR));
    }
    if result.issues.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    Ok(ExitCode::from(EXIT_ISSUES))
}

fn run_explain(id: &str) -> Result<ExitCode> {
    let registry = CheckRegistry::new();
    let check = registry
        .find(id)
        .with_context(|| format!("unknown check id: {id}"))?;
    println!("{} — {}", check.id(), check.name());
    println!("category: {}", check.category());
    println!("severity: {}", check.severity());
    println!();
    println!("{}", check.explanation());
    println!();
    println!("False positives: {}", check.false_positives());
    println!("Tradeoffs: {}", check.tradeoffs());
    println!("Examples: {}", check.metadata().examples);
    println!();
    println!("Remediation: {}", check.remediation());
    Ok(ExitCode::SUCCESS)
}

fn run_init() -> Result<ExitCode> {
    let dest = PathBuf::from("zerum.toml");
    if dest.exists() {
        bail!("{} already exists", dest.display());
    }
    std::fs::write(&dest, include_str!("../zerum.toml.example"))?;
    println!("Wrote {}", dest.display());
    Ok(ExitCode::SUCCESS)
}

fn run_list_checks() -> Result<ExitCode> {
    let registry = CheckRegistry::new();
    for check in registry.iter() {
        println!(
            "{}  {}  [{}] {}",
            check.id(),
            check.name(),
            check.category(),
            check.severity()
        );
    }
    Ok(ExitCode::SUCCESS)
}


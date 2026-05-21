use crate::analyzers::DeterministicAnalyzer;
use crate::config::Config;
use crate::core::CheckRegistry;
use crate::discovery::discover_python_files;
use crate::reporters::{ReportFormat, report};
use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "zerum", version, about = "Deterministic code governance for Python")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run deterministic checks on a path
    Check {
        path: PathBuf,
        #[arg(long)]
        deterministic_only: bool,
        #[arg(long)]
        with_llm: bool,
        #[arg(long, value_enum, default_value_t = ReportFormat::Human)]
        format: ReportFormat,
    },
    /// Alias for check (review hooks reserved for LLM phase)
    Review {
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
    /// List external checkers (orchestration phase)
    ListCheckers,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Check {
                path,
                deterministic_only: _,
                with_llm,
                format,
            } => {
                if with_llm {
                    eprintln!("note: LLM review is not enabled in v0.1.0; running deterministic checks only");
                }
                run_check(&path, format)
            }
            Commands::Review { path, format } => run_check(&path, format),
            Commands::Explain { id } => run_explain(&id),
            Commands::Init => run_init(),
            Commands::ListChecks => run_list_checks(),
            Commands::ListCheckers => run_list_checkers(),
        }
    }
}

fn run_check(path: &Path, format: ReportFormat) -> Result<()> {
    let config = Config::discover(path)?;
    let files = discover_python_files(path)?;
    if files.is_empty() {
        bail!("no Python files found under {}", path.display());
    }
    let analyzer = DeterministicAnalyzer::new();
    let issues = analyzer.analyze_paths(&files, &config)?;
    print!("{}", report(format, &issues)?);
    if issues.is_empty() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn run_explain(id: &str) -> Result<()> {
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
    println!("Remediation: {}", check.remediation());
    Ok(())
}

fn run_init() -> Result<()> {
    let dest = PathBuf::from("zerum.toml");
    if dest.exists() {
        bail!("{} already exists", dest.display());
    }
    std::fs::write(&dest, include_str!("../zerum.toml.example"))?;
    println!("Wrote {}", dest.display());
    Ok(())
}

fn run_list_checks() -> Result<()> {
    let registry = CheckRegistry::new();
    for check in registry.all() {
        println!(
            "{}  {}  [{}] {}",
            check.id(),
            check.name(),
            check.category(),
            check.severity()
        );
    }
    Ok(())
}

fn run_list_checkers() -> Result<()> {
    println!("External checkers (planned): ruff, pylint, mypy, bandit, eslint, credo, sobelow, clippy");
    Ok(())
}

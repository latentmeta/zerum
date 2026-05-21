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
    Markdown,
    Sarif,
}

impl From<ReportFormat> for ReportKind {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Human => ReportKind::Human,
            ReportFormat::Json => ReportKind::Json,
            ReportFormat::Markdown => ReportKind::Markdown,
            ReportFormat::Sarif => ReportKind::Sarif,
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
        #[arg(long)]
        deterministic_only: bool,
        #[arg(long)]
        with_llm: bool,
        #[arg(long, value_enum, default_value_t = ReportFormat::Human)]
        format: ReportFormat,
    },
    /// Run checks and print results without failing on findings (exits 0 unless operational error)
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
    pub fn run(self) -> Result<ExitCode> {
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
                run_check(&path, format, FailOnIssues::Yes)
            }
            Commands::Review { path, format } => run_check(&path, format, FailOnIssues::No),
            Commands::Explain { id } => run_explain(&id),
            Commands::Init => run_init(),
            Commands::ListChecks => run_list_checks(),
            Commands::ListCheckers => run_list_checkers(),
        }
    }
}

enum FailOnIssues {
    Yes,
    No,
}

fn run_check(path: &Path, format: ReportFormat, fail_on_issues: FailOnIssues) -> Result<ExitCode> {
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
    match fail_on_issues {
        FailOnIssues::Yes => Ok(ExitCode::from(EXIT_ISSUES)),
        FailOnIssues::No => Ok(ExitCode::SUCCESS),
    }
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

fn run_list_checkers() -> Result<ExitCode> {
    println!("External checkers (planned): ruff, pylint, mypy, bandit, eslint, credo, sobelow, clippy");
    Ok(ExitCode::SUCCESS)
}

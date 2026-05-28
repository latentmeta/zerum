use crate::analyzers::{DeterministicAnalyzer, ExternalAnalyzer};
use crate::config::Config;
use crate::core::CheckRegistry;
use crate::discovery::discover_python_files;
use crate::integrations::builtin_checkers;
use crate::reporters::{render, ReportKind};
use anyhow::{bail, Context, Result};
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
    #[cfg(feature = "sarif")]
    Sarif,
}

impl From<ReportFormat> for ReportKind {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Human => ReportKind::Human,
            ReportFormat::Json => ReportKind::Json,
            #[cfg(feature = "sarif")]
            ReportFormat::Sarif => ReportKind::Sarif,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "zerum",
    version,
    about = "Deterministic code governance for Python"
)]
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
        /// Config profile name (`default`, `strict`, or custom from zerum.toml)
        #[arg(long)]
        profile: Option<String>,
        /// Run external checker(s), comma-separated (e.g. `ruff`)
        #[arg(long, value_delimiter = ',')]
        with_external: Vec<String>,
    },
    /// Explain a check by id (e.g. ZR001)
    Explain { id: String },
    /// Write a starter zerum.toml in the current directory
    Init {
        /// Write strict profile example (`zerum.toml.strict.example`) instead of default
        #[arg(long)]
        strict: bool,
    },
    /// List built-in deterministic checks
    ListChecks,
    /// List configured and available external checkers
    ListCheckers,
}

impl Cli {
    pub fn run(self) -> Result<ExitCode> {
        match self.command {
            Commands::Check {
                path,
                format,
                profile,
                with_external,
            } => run_check(&path, format, profile.as_deref(), with_external),
            Commands::Explain { id } => run_explain(&id),
            Commands::Init { strict } => run_init(strict),
            Commands::ListChecks => run_list_checks(),
            Commands::ListCheckers => run_list_checkers(),
        }
    }
}

fn apply_profile_override(mut config: Config, profile: Option<&str>) -> Config {
    if let Some(name) = profile {
        config.profile.name = name.to_string();
    }
    config
}

fn run_check(
    path: &Path,
    format: ReportFormat,
    profile: Option<&str>,
    with_external: Vec<String>,
) -> Result<ExitCode> {
    let mut config = Config::discover(path)?;
    config = apply_profile_override(config, profile);

    let files = discover_python_files(path)?;
    if files.is_empty() {
        bail!("no Python files found under {}", path.display());
    }

    let analyzer = DeterministicAnalyzer::new();
    let mut result = analyzer.analyze_paths(&files, &config);

    let mut external_ids = config.external_checkers.clone();
    external_ids.extend(with_external);
    if !external_ids.is_empty() {
        let ext = ExternalAnalyzer::with_checkers(external_ids);
        let mut external_issues = ext.run(path)?;
        result.issues.append(&mut external_issues);
        result.issues.sort_by(|a, b| {
            (&a.file, a.line, a.column, &a.id, &a.message)
                .cmp(&(&b.file, b.line, b.column, &b.id, &b.message))
        });
    }

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

fn run_init(strict: bool) -> Result<ExitCode> {
    let dest = PathBuf::from("zerum.toml");
    if dest.exists() {
        bail!("{} already exists", dest.display());
    }
    let template = if strict {
        include_str!("../zerum.toml.strict.example")
    } else {
        include_str!("../zerum.toml.example")
    };
    std::fs::write(&dest, template)?;
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
    for checker in builtin_checkers() {
        let status = if checker.is_available() {
            "available"
        } else {
            "not installed"
        };
        println!("{}  {}  [{status}]", checker.id(), checker.name());
    }
    Ok(ExitCode::SUCCESS)
}

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::DirEntry;
use walkdir::WalkDir;

const SKIP_DIRS: &[&str] = &[
    ".git",
    ".venv",
    "venv",
    "__pycache__",
    "node_modules",
    "target",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
];

fn skip_entry(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && entry
            .file_name()
            .to_str()
            .is_some_and(|name| SKIP_DIRS.contains(&name))
}

pub fn discover_python_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if root.is_file() {
        if is_python_file(root) {
            files.push(root.to_path_buf());
        }
        return Ok(files);
    }

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !skip_entry(e))
    {
        let entry = entry.with_context(|| format!("walk directory {}", root.display()))?;
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        if is_python_file(path) {
            files.push(path.to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

fn is_python_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("py")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn skips_git_and_pycache_directories() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/hooks.py"), "def bad(): pass\n").unwrap();
        fs::create_dir_all(root.join("__pycache__")).unwrap();
        fs::write(root.join("__pycache__/cached.py"), "def bad(): pass\n").unwrap();
        fs::write(root.join("real.py"), "def ok(): pass\n").unwrap();

        let files = discover_python_files(root).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("real.py"));
    }

    #[test]
    fn errors_when_root_is_missing() {
        let missing = Path::new("/nonexistent/zerum/discovery/root");
        let err = discover_python_files(missing).unwrap_err();
        assert!(err.to_string().contains("walk directory"));
    }
}

//! External tool adapters.

pub mod external_checker;
pub mod ruff;

pub use external_checker::{builtin_checkers, find_checker, ExternalChecker, ExternalFinding};
pub use ruff::RuffChecker;

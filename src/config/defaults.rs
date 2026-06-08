//! Built-in profile defaults applied when `zerum.toml` has no per-check override.

/// Checks disabled in the built-in `default` profile (noisy heuristics / project-context rules).
pub const DEFAULT_PROFILE_DISABLED: &[&str] = &[
    // Readability — docstrings and comment patterns are noisy on small modules
    "ZR007", "ZR008", "ZR011", "ZR012", "ZR013", "ZR014", "ZR015",
    // Consistency — pattern heuristics (ZR105/106 are AST-precise and stay on)
    "ZR102", "ZR103", "ZR104", "ZR107", "ZR108", "ZR109", "ZR110",
    // Design — pattern heuristics (ZR201/205/207/209 stay on or need config)
    "ZR202", "ZR203", "ZR204", "ZR206", "ZR208", "ZR210",
    // AI — pattern-heavy slop rules (ZR501/502/504/507/508 are AST-precise)
    "ZR503", "ZR505", "ZR506", "ZR509", "ZR510",
];

pub fn is_disabled_in_default_profile(id: &str) -> bool {
    DEFAULT_PROFILE_DISABLED.contains(&id)
}

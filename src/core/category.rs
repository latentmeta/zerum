use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Readability,
    Consistency,
    Refactor,
    Design,
    Warning,
    Security,
    Architecture,
    Ai,
    Performance,
    Maintainability,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Readability => write!(f, "readability"),
            Category::Consistency => write!(f, "consistency"),
            Category::Refactor => write!(f, "refactor"),
            Category::Design => write!(f, "design"),
            Category::Warning => write!(f, "warning"),
            Category::Security => write!(f, "security"),
            Category::Architecture => write!(f, "architecture"),
            Category::Ai => write!(f, "ai"),
            Category::Performance => write!(f, "performance"),
            Category::Maintainability => write!(f, "maintainability"),
        }
    }
}

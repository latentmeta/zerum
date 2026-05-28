use crate::checks::catalog::build_catalog;
use crate::core::Check;

pub struct CheckRegistry {
    checks: Vec<Box<dyn Check>>,
}

impl Default for CheckRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckRegistry {
    pub fn new() -> Self {
        let checks: Vec<Box<dyn Check>> = build_catalog();
        Self { checks }
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Check> {
        self.checks.iter().map(|c| c.as_ref())
    }

    #[must_use]
    pub fn find(&self, id: &str) -> Option<&dyn Check> {
        self.checks
            .iter()
            .find(|c| c.id() == id)
            .map(|c| c.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_unique_zr_ids() {
        let registry = CheckRegistry::new();
        let mut ids: Vec<_> = registry.iter().map(|c| c.id()).collect();
        ids.sort_unstable();
        assert!(ids.len() >= 75);
        for id in &ids {
            assert!(id.starts_with("ZR"));
            assert_eq!(id.len(), 5);
        }
        assert_eq!(ids.windows(2).filter(|w| w[0] == w[1]).count(), 0);
    }

    #[test]
    fn registry_metadata_is_complete_and_aligned() {
        let registry = CheckRegistry::new();
        for check in registry.iter() {
            let meta = check.metadata();
            assert_eq!(meta.id, check.id());
            assert_eq!(meta.name, check.name());
            assert_eq!(meta.category, check.category());
            assert_eq!(meta.severity, check.severity());
            assert!(!meta.examples.trim().is_empty());
        }
    }
}

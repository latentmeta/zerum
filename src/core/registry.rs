use crate::checks::{
    ai::placeholder_comment::AiPlaceholderComment,
    architecture::forbidden_imports::ForbiddenArchitectureImport,
    design::nested_conditionals::NestedConditionals,
    readability::{long_function::LongFunction, too_many_arguments::TooManyArguments},
    warning::{broad_except::BroadExcept, mutable_default::MutableDefaultArgument, print_debugging::PrintDebugging},
    Zr001TooManyBranches, Zr008GodClass,
};
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
        let checks: Vec<Box<dyn Check>> = vec![
            Box::new(Zr001TooManyBranches),
            Box::new(TooManyArguments),
            Box::new(LongFunction),
            Box::new(NestedConditionals),
            Box::new(BroadExcept),
            Box::new(PrintDebugging),
            Box::new(MutableDefaultArgument),
            Box::new(Zr008GodClass),
            Box::new(AiPlaceholderComment),
            Box::new(ForbiddenArchitectureImport),
        ];
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
        assert_eq!(ids.len(), 10);
        for id in &ids {
            assert!(id.starts_with("ZR"));
            assert_eq!(id.len(), 5);
        }
        assert_eq!(ids.windows(2).filter(|w| w[0] == w[1]).count(), 0);
    }
}

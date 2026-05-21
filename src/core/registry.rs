use crate::checks::{
    ai::placeholder_comment::AiPlaceholderComment,
    architecture::forbidden_imports::ForbiddenArchitectureImport,
    design::nested_conditionals::NestedConditionals,
    readability::{long_function::LongFunction, too_many_arguments::TooManyArguments},
    warning::{broad_except::BroadExcept, mutable_default::MutableDefaultArgument, print_debugging::PrintDebugging},
    Zr001TooManyBranches, Zr008GodClass,
};
use crate::core::Check;
use std::sync::Arc;

pub struct CheckRegistry {
    checks: Vec<Arc<dyn Check>>,
}

impl Default for CheckRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckRegistry {
    pub fn new() -> Self {
        let checks: Vec<Arc<dyn Check>> = vec![
            Arc::new(Zr001TooManyBranches),
            Arc::new(TooManyArguments),
            Arc::new(LongFunction),
            Arc::new(NestedConditionals),
            Arc::new(BroadExcept),
            Arc::new(PrintDebugging),
            Arc::new(MutableDefaultArgument),
            Arc::new(Zr008GodClass),
            Arc::new(AiPlaceholderComment),
            Arc::new(ForbiddenArchitectureImport),
        ];
        Self { checks }
    }

    pub fn all(&self) -> &[Arc<dyn Check>] {
        &self.checks
    }

    #[must_use]
    pub fn find(&self, id: &str) -> Option<&Arc<dyn Check>> {
        self.checks.iter().find(|c| c.id() == id)
    }
}

use crate::core::{Category, CheckContext, Issue, Severity};

#[derive(Debug, Clone, Copy)]
pub struct CheckMetadata {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub severity: Severity,
    pub safe_fixable: bool,
    pub examples: &'static str,
}

pub trait Check: Send + Sync {
    fn metadata(&self) -> CheckMetadata {
        CheckMetadata {
            id: self.id(),
            name: self.name(),
            category: self.category(),
            severity: self.severity(),
            safe_fixable: false,
            examples: "",
        }
    }

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;
    fn severity(&self) -> Severity;
    fn explanation(&self) -> &'static str;
    fn remediation(&self) -> &'static str;
    fn false_positives(&self) -> &'static str {
        "This rule may trigger on project-specific styles."
    }
    fn tradeoffs(&self) -> &'static str {
        "Disabling this rule may reduce guidance consistency."
    }
    fn run(&self, ctx: &CheckContext) -> Vec<Issue>;
}

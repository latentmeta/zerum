use crate::core::{Category, CheckContext, Issue, Severity};

pub trait Check: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;
    fn severity(&self) -> Severity;
    fn explanation(&self) -> &'static str;
    fn remediation(&self) -> &'static str;
    fn run(&self, ctx: &CheckContext) -> Vec<Issue>;
}

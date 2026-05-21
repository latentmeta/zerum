//! Provider trait and implementations — Phase 4.

pub trait LlmProvider {
    fn name(&self) -> &str;
}

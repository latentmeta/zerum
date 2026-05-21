use crate::core::Issue;
use crate::reporters::Reporter;
use anyhow::Result;

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn report(&self, issues: &[Issue]) -> Result<String> {
        Ok(serde_json::to_string_pretty(issues)?)
    }
}

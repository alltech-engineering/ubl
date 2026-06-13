// Peppol Rules Engine — Schematron-style business rule validation.
//
// Peppol BIS defines 200+ business rules that go beyond XSD validation.
// These rules check things like:
//   - "Invoice total must equal sum of line totals plus tax"
//   - "Supplier MUST have a Peppol participant ID"
//   - "If InvoiceTypeCode is 381, document MUST contain CreditNoteTypeCode"
//
// This module provides a lightweight rule engine that can evaluate
// these rules against domain types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity of a rule violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Document must be rejected — mandatory rule violated.
    Fatal,
    /// Document should be rejected — conditional rule violated.
    Error,
    /// Warning — best practice not followed.
    Warning,
}

/// The outcome of evaluating a single rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOutcome {
    /// The rule identifier (e.g., "PEPPOL-EN16931-R001")
    pub rule_id: String,
    /// Severity of the violation, or None if the rule passed.
    pub severity: Option<Severity>,
    /// Human-readable description of the outcome.
    pub message: String,
    /// The XPath or field path that triggered this outcome.
    pub location: Option<String>,
}

impl RuleOutcome {
    pub fn passed(rule_id: &str) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            severity: None,
            message: "OK".into(),
            location: None,
        }
    }

    pub fn failed(rule_id: &str, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            severity: Some(severity),
            message: message.into(),
            location: None,
        }
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }

    pub fn is_ok(&self) -> bool {
        self.severity.is_none()
    }
}

/// A single Peppol business rule.
pub struct Rule {
    /// Unique identifier (e.g., "PEPPOL-EN16931-R001")
    pub id: String,
    /// Human-readable description of what this rule checks.
    pub description: String,
    /// The severity if this rule fails.
    pub severity: Severity,
    /// The validation function — returns Ok(()) if passed, Err(msg) if violated.
    /// Boxed to allow different closure types.
    pub check: Box<dyn Fn() -> Result<(), String> + Send + Sync>,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rule")
            .field("id", &self.id)
            .field("description", &self.description)
            .finish()
    }
}

impl Rule {
    /// Evaluate this rule and return the outcome.
    pub fn evaluate(&self) -> RuleOutcome {
        match (self.check)() {
            Ok(()) => RuleOutcome::passed(&self.id),
            Err(msg) => RuleOutcome::failed(&self.id, self.severity.clone(), msg),
        }
    }
}

/// A collection of Peppol business rules that can be evaluated together.
#[derive(Default)]
pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the engine.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Evaluate all rules and return outcomes.
    pub fn evaluate_all(&self) -> Vec<RuleOutcome> {
        self.rules.iter().map(|r| r.evaluate()).collect()
    }

    /// Evaluate all rules and return only failures.
    pub fn evaluate_failures(&self) -> Vec<RuleOutcome> {
        self.evaluate_all()
            .into_iter()
            .filter(|o| !o.is_ok())
            .collect()
    }

    /// Check if all rules pass.
    pub fn validate(&self) -> Result<(), Vec<RuleOutcome>> {
        let failures: Vec<_> = self.evaluate_failures();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_pass() {
        let rule = Rule {
            id: "TEST-001".into(),
            description: "Always passes".into(),
            severity: Severity::Error,
            check: Box::new(|| Ok(())),
        };
        let outcome = rule.evaluate();
        assert!(outcome.is_ok());
    }

    #[test]
    fn test_rule_fail() {
        let rule = Rule {
            id: "TEST-002".into(),
            description: "Always fails".into(),
            severity: Severity::Fatal,
            check: Box::new(|| Err("test failure".into())),
        };
        let outcome = rule.evaluate();
        assert!(!outcome.is_ok());
        assert_eq!(outcome.severity, Some(Severity::Fatal));
    }

    #[test]
    fn test_engine() {
        let mut engine = RuleEngine::new();
        engine.add_rule(Rule {
            id: "A".into(), description: "p".into(), severity: Severity::Error,
            check: Box::new(|| Ok(())),
        });
        engine.add_rule(Rule {
            id: "B".into(), description: "f".into(), severity: Severity::Warning,
            check: Box::new(|| Err("oops".into())),
        });

        let failures = engine.evaluate_failures();
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].rule_id, "B");
    }
}

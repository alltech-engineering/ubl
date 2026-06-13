// Peppol BIS Billing 3.0 Validator
//
// High-level validation orchestration — runs XSD validation,
// Peppol business rules, and national rules in sequence.

use peppol_common::rules::{RuleOutcome, RuleEngine};
use ubl_documents::billing::Invoice;

/// Validate an Invoice against Peppol BIS Billing 3.0.
/// Returns all rule outcomes (both passes and failures).
pub fn validate_invoice(invoice: &Invoice) -> Vec<RuleOutcome> {
    let engine = super::rules::billing_rules(invoice);
    engine.evaluate_all()
}

/// Quick validation — returns only failures.
pub fn validate_invoice_quick(invoice: &Invoice) -> Vec<RuleOutcome> {
    let engine = super::rules::billing_rules(invoice);
    engine.evaluate_failures()
}

/// Validate an Invoice with South African national rules applied.
pub fn validate_invoice_za(invoice: &Invoice) -> Vec<RuleOutcome> {
    let mut engine = super::rules::billing_rules(invoice);

    // Add ZA-specific rules
    for rule in super::za::national_rules(invoice) {
        engine.add_rule(rule);
    }

    engine.evaluate_all()
}

/// Check if an invoice would pass all fatal rules.
pub fn is_compliant(invoice: &Invoice) -> bool {
    let failures = validate_invoice_quick(invoice);
    !failures.iter().any(|f| {
        matches!(f.severity, Some(peppol_common::rules::Severity::Fatal))
    })
}

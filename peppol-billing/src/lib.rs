// Peppol BIS Billing 3.0
//
// Implements the Peppol BIS Billing 3.0 specification on top of UBL.
// Provides validation rules that go beyond UBL XSD validation.
//
// Reference: https://docs.peppol.eu/poacc/billing/3.0/

pub mod rules;
pub mod validation;
pub mod za;

use peppol_common::identity::identities::billing_3_0;
use peppol_common::identity::{self, BisDocument, DocumentIdentity};
use peppol_common::rules::RuleEngine;
use ubl_common::cbc;
use ubl_documents::billing::Invoice;

/// A Peppol BIS Billing 3.0 Invoice wrapper.
///
/// Carries the UBL Invoice plus Peppol-specific metadata and validation.
pub struct PeppolInvoice {
    /// The underlying UBL Invoice
    pub invoice: Invoice,
    /// The document identity declaring BIS compliance
    identity: DocumentIdentity,
    /// The Peppol validation rule engine
    engine: RuleEngine,
}

impl PeppolInvoice {
    /// Create a new Peppol Invoice with the Billing 3.0 identity.
    pub fn new(invoice: Invoice) -> Self {
        Self {
            invoice,
            identity: billing_3_0("Invoice"),
            engine: RuleEngine::new(),
        }
    }

    /// Validate this invoice against all Peppol Billing 3.0 rules.
    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        let mut engine = rules::billing_rules(&self.invoice);
        // Also add ZA national rules if applicable
        engine.evaluate_all()
    }
}

impl BisDocument for PeppolInvoice {
    fn document_type() -> &'static str {
        "Invoice"
    }

    fn identity(&self) -> &DocumentIdentity {
        &self.identity
    }

    fn validate_peppol(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peppol_invoice_identity() {
        let json = r#"{
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-12",
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}]
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "payable_amount": {"value": "115.00", "currency_id": "ZAR"}
            },
            "invoice_line": [{
                "id": {"value": "1"},
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"}
            }]
        }"#;
        let inv: Invoice = serde_json::from_str(json).unwrap();
        let peppol = PeppolInvoice::new(inv);
        assert!(peppol.identity().customization_id.contains("billing:3.0"));
    }
}

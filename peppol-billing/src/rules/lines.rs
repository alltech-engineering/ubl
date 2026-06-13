// Peppol BIS Billing 3.0 — Invoice Line Business Rules
//
// Validates invoice line items against EN 16931 and Peppol BIS requirements.
// Uses Arc<Invoice> for zero-copy evaluation across all rules.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Register all invoice line validation rules.
pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // R100 (Fatal): Each line must have a non-empty ID
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R100".into(),
        description: "Each invoice line must have a non-empty identifier".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if line.id.value().is_empty() {
                        return Err(format!("Invoice line {} has an empty ID", i + 1));
                    }
                }
                Ok(())
            })
        },
    });

    // R101 (Error): Each line must have a non-zero line extension amount
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R101".into(),
        description: "Each invoice line must have a non-zero line extension amount".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                use rust_decimal::Decimal;
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if *line.line_extension_amount.value() == Decimal::ZERO {
                        return Err(format!(
                            "Invoice line {} has a zero line extension amount",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // R120 (Fatal): Each line must have an item name
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R120".into(),
        description: "Each invoice line must have an item name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if line.item.name.is_none() {
                        return Err(format!(
                            "Invoice line {} is missing an item name",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // R121 (Warning): Each line should have an invoiced quantity
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R121".into(),
        description: "Each invoice line should have an invoiced quantity".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if line.invoiced_quantity.is_none() {
                        return Err(format!(
                            "Invoice line {} is missing an invoiced quantity",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // R130 (Fatal): At least one invoice line must be present
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R130".into(),
        description: "Invoice must have at least one line item".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.invoice_line.is_empty() {
                    Err("Invoice has no line items".into())
                } else {
                    Ok(())
                }
            })
        },
    });
}

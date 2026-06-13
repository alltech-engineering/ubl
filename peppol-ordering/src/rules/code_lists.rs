// Peppol BIS Ordering 3.0 — Code List Validation Rules
//
// Validates that coded values belong to the correct ISO / UNCL code lists.

use peppol_common::codes::{currency_codes, payment_means_codes};
use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-CL001 (Fatal): DocumentCurrencyCode must be valid ISO 4217 ────
    engine.add_rule(Rule {
        id: "ORD-CL001".into(),
        description: "DocumentCurrencyCode must be a valid ISO 4217 currency code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_currency_code {
                None => Err("DocumentCurrencyCode is missing — required for Peppol BIS".into()),
                Some(cc) => {
                    let code = cc.value();
                    if currency_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "DocumentCurrencyCode '{}' is not a valid ISO 4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── ORD-CL002 (Fatal): PaymentMeansCode must be from UNCL 4461 ────────
    engine.add_rule(Rule {
        id: "ORD-CL002".into(),
        description: "PaymentMeansCode must be a valid UNCL 4461 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    let code = pm.payment_means_code.value();
                    if !payment_means_codes().is_valid(code) {
                        return Err(format!(
                            "PaymentMeans[{}] code '{}' is not a valid UNCL4461 code",
                            i + 1,
                            code
                        ));
                    }
                }
                Ok(())
            })
        },
    });
}

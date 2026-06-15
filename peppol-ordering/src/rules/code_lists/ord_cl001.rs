/// ORD-CL001 (Fatal): DocumentCurrencyCode must be valid ISO 4217
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;
use peppol_common::codes::{currency_codes};

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
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
    }
}

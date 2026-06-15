/// ORD-CL003 (Fatal): PricingCurrencyCode must be valid ISO 4217 if present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;
use peppol_common::codes::{currency_codes};

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-CL003".into(),
        description: "PricingCurrencyCode must be a valid ISO 4217 currency code if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.pricing_currency_code {
                None => Ok(()),
                Some(cc) => {
                    let code = cc.value();
                    if code.is_empty() {
                        Err("PricingCurrencyCode is present but empty".into())
                    } else if currency_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "PricingCurrencyCode '{}' is not a valid ISO 4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    }
}

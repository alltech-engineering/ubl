/// ORD-R042 (Fatal): PaymentMeans payment_means_code must be valid
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R042".into(),
        description: "PaymentMeans payment_means_code must be valid (non-empty)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    let code = pm.payment_means_code.value();
                    if code.is_empty() {
                        return Err(format!(
                            "PaymentMeans[{}] has an empty payment_means_code — a valid code is required",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

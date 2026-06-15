/// ORD-R044 (Warning): PaymentTerms should include settlement period
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R044".into(),
        description: "PaymentTerms should include settlement period".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pt) in inv.payment_terms.iter().enumerate() {
                    if pt.settlement_period.is_none() {
                        return Err(format!(
                            "PaymentTerms[{}] has no settlement period — consider specifying when payment is due",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

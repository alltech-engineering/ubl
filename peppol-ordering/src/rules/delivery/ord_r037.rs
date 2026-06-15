/// ORD-R037 (Warning): DeliveryTerms should include special instructions
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R037".into(),
        description: "DeliveryTerms should include special instructions".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, dt) in inv.delivery_terms.iter().enumerate() {
                    if dt.special_terms.is_empty() {
                        return Err(format!(
                            "DeliveryTerms[{}] has no special terms — consider including delivery instructions",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

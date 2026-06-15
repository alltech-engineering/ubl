/// ORD-R045 (Warning): PaymentTerms note for discount/penalty information
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R045".into(),
        description: "PaymentTerms should include discount/penalty information in notes".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pt) in inv.payment_terms.iter().enumerate() {
                    if pt.note.is_empty() {
                        return Err(format!(
                            "PaymentTerms[{}] has no notes — consider specifying discount or penalty information",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

/// ORD-R041 (Warning): Payment terms should be specified
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R041".into(),
        description: "Payment terms should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.payment_terms.is_empty() {
                    Err("No payment terms specified — consider defining payment timing and conditions".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}

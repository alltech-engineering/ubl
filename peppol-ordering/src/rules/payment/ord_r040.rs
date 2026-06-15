/// ORD-R040 (Warning): Payment means should be specified
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R040".into(),
        description: "Payment means should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.payment_means.is_empty() {
                    Err("No payment means specified — consider defining how payment should be made".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}

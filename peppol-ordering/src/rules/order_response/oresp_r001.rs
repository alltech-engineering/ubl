/// ORESP-R001 (Fatal): OrderResponse ID must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R001".into(),
        description: "OrderResponse ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.id.value().is_empty() {
                    Err("OrderResponse ID is empty — a non-empty identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}

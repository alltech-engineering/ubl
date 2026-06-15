/// ORD-R005 (Warning): Notes should provide context
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R005".into(),
        description: "Notes should provide context".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.note.is_empty() {
                    Err("No order notes provided — consider adding context about the order".into())
                } else if inv.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Order notes are present but all are empty — provide meaningful context"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    }
}

/// ORESP-R009 (Warning): Rejection/changes should document reason
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;
use super::is_rejection_or_change;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R009".into(),
        description: "If order is rejected or changed, reason should be documented".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                Some(code) if is_rejection_or_change(code.value()) => {
                    if inv.note.is_empty() || inv.note.iter().all(|n| n.value().trim().is_empty()) {
                        Err("OrderResponse indicates rejection or change but no meaningful notes document the reason".into())
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            })
        },
    }
}

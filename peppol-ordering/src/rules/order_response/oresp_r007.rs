/// ORESP-R007 (Warning): Notes should explain changes/rejections
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R007".into(),
        description: "Notes should explain any changes or rejections".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.note.is_empty() {
                    Err("No notes provided — consider adding explanation of the response".into())
                } else if inv.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Notes are present but all are empty — provide meaningful explanation"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    }
}

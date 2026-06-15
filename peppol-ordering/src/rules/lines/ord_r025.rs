/// ORD-R025 (Warning): Line note should be present for context
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R025".into(),
        description: "Line note should be present for context".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    let notes = if let Some(ref li) = line.line_item {
                        if li.note.is_empty() { &line.note } else { return Ok(()) }
                    } else {
                        &line.note
                    };
                    if notes.is_empty() {
                        return Err(format!(
                            "Order line {} has no note — consider adding context",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

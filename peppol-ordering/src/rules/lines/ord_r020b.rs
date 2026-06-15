/// ORD-R020b (Fatal): Each OrderLine must have a LineItem
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R020b".into(),
        description: "Each OrderLine must have a LineItem".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.line_item.is_none() {
                        return Err(format!(
                            "Order line {} has no LineItem",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

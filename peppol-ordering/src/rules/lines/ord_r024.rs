/// ORD-R024 (Error): Each line item should have a price
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R024".into(),
        description: "Each line should have a price".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if li.price.is_none() {
                            return Err(format!(
                                "Order line {} has no price specified",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    }
}

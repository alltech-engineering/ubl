/// ORD-R023 (Fatal): Each line item must have an ordered quantity
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R023".into(),
        description: "Each line must have an ordered quantity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if li.quantity.is_none() {
                            return Err(format!(
                                "Order line {} is missing an ordered quantity",
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

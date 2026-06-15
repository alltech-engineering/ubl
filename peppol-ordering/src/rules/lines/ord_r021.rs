/// ORD-R021 (Fatal): Each line item must have a non-empty ID
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R021".into(),
        description: "Each line must have a non-empty ID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        match &li.id {
                            None => {
                                return Err(format!(
                                    "Order line {} LineItem has no ID",
                                    i + 1
                                ));
                            }
                            Some(id) if id.value().is_empty() => {
                                return Err(format!(
                                    "Order line {} LineItem has an empty ID",
                                    i + 1
                                ));
                            }
                            Some(_) => {}
                        }
                    }
                }
                Ok(())
            })
        },
    }
}

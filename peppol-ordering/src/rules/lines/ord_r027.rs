/// ORD-R027 (Fatal): LineItem must have LineExtensionAmount
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R027".into(),
        description: "LineItem must have LineExtensionAmount (line total)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if li.line_extension_amount.is_none() {
                            return Err(format!(
                                "Order line {} has no LineExtensionAmount — line total is required",
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

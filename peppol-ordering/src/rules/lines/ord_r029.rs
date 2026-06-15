/// ORD-R029 (Fatal): Item identification should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R029".into(),
        description: "Item identification (SellersItemIdentification) should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let Some(ref item) = li.item {
                            if item.sellers_item_identification.is_none() {
                                return Err(format!(
                                    "Order line {} has no SellersItemIdentification — item identification is required",
                                    i + 1
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    }
}

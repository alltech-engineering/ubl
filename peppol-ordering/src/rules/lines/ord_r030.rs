/// ORD-R030 (Warning): Item classification should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R030".into(),
        description: "Item classification (commodity code) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let Some(ref item) = li.item {
                            if item.commodity_classification.is_empty() {
                                return Err(format!(
                                    "Order line {} has no commodity classification — consider adding commodity code",
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

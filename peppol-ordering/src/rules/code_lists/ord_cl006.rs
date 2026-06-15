/// ORD-CL006 (Warning): Unit codes in quantities should be from UN/ECE Rec.20
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-CL006".into(),
        description: "Unit codes in quantities should be from UN/ECE Rec.20".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let Some(ref qty) = li.quantity {
                            match &qty.unit_code {
                                None => {
                                    return Err(format!(
                                        "Order line {} quantity has no unit code — unit should be specified from UN/ECE Rec.20",
                                        i + 1
                                    ));
                                }
                                Some(unit) if unit.is_empty() => {
                                    return Err(format!(
                                        "Order line {} quantity has an empty unit code — unit should be from UN/ECE Rec.20",
                                        i + 1
                                    ));
                                }
                                Some(_) => {}
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    }
}

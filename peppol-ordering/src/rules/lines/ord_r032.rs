/// ORD-R032 (Fatal): BaseQuantity unit must match quantity unit
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R032".into(),
        description: "BaseQuantity for price must match order quantity unit".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let (Some(qty), Some(price)) = (&li.quantity, &li.price) {
                            if let Some(ref base_qty) = price.base_quantity {
                                let qty_unit = qty.unit_code.as_deref().unwrap_or("");
                                let base_unit = base_qty.0.unit_code.as_deref().unwrap_or("");
                                if qty_unit != base_unit {
                                    return Err(format!(
                                        "Order line {} price base quantity unit '{}' does not match order quantity unit '{}'",
                                        i + 1, base_unit, qty_unit
                                    ));
                                }
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    }
}

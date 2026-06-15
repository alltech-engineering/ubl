/// ORD-R038 (Warning): Shipment information should include transport details
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R038".into(),
        description: "Shipment information should include transport details".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(shipment) = &delivery.shipment {
                        if shipment.information.is_empty() && shipment.goods_item.is_empty() {
                            return Err(format!(
                                "Delivery[{}] shipment has no transport details — consider adding information or goods items",
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

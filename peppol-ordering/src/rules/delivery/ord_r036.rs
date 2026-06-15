/// ORD-R036 (Fatal): RequestedDeliveryPeriod start/end must be valid dates
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R036".into(),
        description: "RequestedDeliveryPeriod start/end must be valid dates".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(period) = &delivery.requested_delivery_period {
                        if let (Some(start), Some(end)) = (&period.start_date, &period.end_date) {
                            if start.0 > end.0 {
                                return Err(format!(
                                    "Delivery[{}] requested period start {} is after end {}",
                                    i + 1,
                                    start.0.format("%Y-%m-%d"),
                                    end.0.format("%Y-%m-%d")
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

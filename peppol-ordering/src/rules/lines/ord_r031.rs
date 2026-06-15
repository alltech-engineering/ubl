/// ORD-R031 (Fatal): Price amount must be present and positive
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R031".into(),
        description: "Price amount must be present for each line".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        match &li.price {
                            None => {
                                return Err(format!(
                                    "Order line {} has no price — price amount is required",
                                    i + 1
                                ));
                            }
                            Some(price) => {
                                if *price.price_amount.value() == rust_decimal::Decimal::ZERO {
                                    return Err(format!(
                                        "Order line {} has zero price amount — a positive price is required",
                                        i + 1
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

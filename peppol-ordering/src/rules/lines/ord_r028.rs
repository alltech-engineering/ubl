/// ORD-R028 (Error): Line total must equal quantity * price
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R028".into(),
        description: "Line total must equal quantity * price".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let (Some(qty), Some(line_ext), Some(price)) =
                            (&li.quantity, &li.line_extension_amount, &li.price)
                        {
                            let expected = qty.value * price.price_amount.value();
                            let diff = (line_ext.value - expected).abs();
                            if diff > rust_decimal::Decimal::new(2, 2) {
                                return Err(format!(
                                    "Order line {} line total {} does not match quantity {} * price {} = {}",
                                    i + 1,
                                    line_ext.value,
                                    qty.value,
                                    price.price_amount.value(),
                                    expected
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

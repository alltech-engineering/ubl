/// ORESP-R006 (Fatal): Response code must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R006".into(),
        description: "Response code must be present (OrderResponseCode)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                None => Err("OrderResponseCode is missing — a response code is required".into()),
                Some(code) if code.value().is_empty() => {
                    Err("OrderResponseCode is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    }
}

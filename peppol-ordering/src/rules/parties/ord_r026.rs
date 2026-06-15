/// ORD-R026 (Warning): OriginatorCustomerParty should be present for drop-ship scenarios
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R026".into(),
        description: "OriginatorCustomerParty should be present for drop-ship scenarios".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.originator_customer_party.is_none() {
                    Err("OriginatorCustomerParty is not present — should be specified for drop-ship scenarios".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}

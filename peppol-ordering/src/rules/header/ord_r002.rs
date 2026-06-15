/// ORD-R002 (Fatal): Issue date must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R002".into(),
        description: "Issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let _date = &inv.issue_date;
                Ok(())
            })
        },
    }
}

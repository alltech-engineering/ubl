/// ORD-R013 (Warning): Issue date must not be in the future
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R013".into(),
        description: "Issue date must not be in the future".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let today = chrono::Utc::now().date_naive();
                if inv.issue_date.0 > today {
                    Err(format!(
                        "Issue date {} is in the future — document should not be post-dated",
                        inv.issue_date.0.format("%Y-%m-%d")
                    ))
                } else {
                    Ok(())
                }
            })
        },
    }
}

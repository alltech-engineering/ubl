/// ORD-R003 (Fatal): Document currency code must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R003".into(),
        description: "Document currency code must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_currency_code {
                None => Err("Document currency code is missing — required for Peppol BIS".into()),
                Some(cc) if cc.value().is_empty() => {
                    Err("Document currency code is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    }
}

/// ORD-R014 (Fatal): AccountingCostCode must be valid if present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R014".into(),
        description: "AccountingCostCode must be valid if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_cost_code {
                None => Ok(()),
                Some(code) if code.value().is_empty() => {
                    Err("AccountingCostCode is present but empty — must be a valid code".into())
                }
                Some(_) => Ok(()),
            })
        },
    }
}

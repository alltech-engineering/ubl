/// ORD-R011 (Fatal): Contract reference ID must be present if contract is specified
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R011".into(),
        description: "Contract reference ID must be present if contract is specified".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, contract) in inv.contract.iter().enumerate() {
                    match &contract.id {
                        None => {
                            return Err(format!(
                                "Contract[{}] has no ID — a contract reference ID is required",
                                i + 1
                            ));
                        }
                        Some(id) if id.value().is_empty() => {
                            return Err(format!(
                                "Contract[{}] ID is empty — a non-empty contract reference is required",
                                i + 1
                            ));
                        }
                        Some(_) => {}
                    }
                }
                Ok(())
            })
        },
    }
}

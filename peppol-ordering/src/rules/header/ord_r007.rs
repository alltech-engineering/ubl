/// ORD-R007 (Fatal): OriginatorDocumentReference must reference prior order or contract if present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R007".into(),
        description: "OriginatorDocumentReference must reference a prior order or contract if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(originator) = &inv.originator_document_reference {
                    match &originator.id {
                        None => Err("OriginatorDocumentReference is present but has no ID — must reference a prior order or contract".into()),
                        Some(id) if id.value().is_empty() => {
                            Err("OriginatorDocumentReference ID is empty — must reference a prior order or contract".into())
                        }
                        Some(_) => Ok(()),
                    }
                } else {
                    Ok(())
                }
            })
        },
    }
}

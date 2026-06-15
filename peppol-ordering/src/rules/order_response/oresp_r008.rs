/// ORESP-R008 (Fatal): Accepted orders must have lines
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;
use super::is_acceptance;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R008".into(),
        description: "If order is accepted, at least one OrderLine should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                Some(code) if is_acceptance(code.value()) => {
                    if inv.order_line.is_empty() {
                        Err("OrderResponse indicates acceptance but no OrderLines are present — at least one accepted line is required".into())
                    } else {
                        Ok(())
                    }
                }
                _ => {
                    // Not an acceptance; lines may be empty for rejections
                    Ok(())
                }
            })
        },
    }
}

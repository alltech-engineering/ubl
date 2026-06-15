/// ORD-R033 (Warning): AllowanceCharge should have reason code
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R033".into(),
        description: "AllowanceCharge at line level should have reason code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        for (j, ac) in li.allowance_charge.iter().enumerate() {
                            if ac.allowance_charge_reason_code.is_none() {
                                return Err(format!(
                                    "Order line {} AllowanceCharge[{}] has no reason code — reason should be specified",
                                    i + 1, j + 1
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

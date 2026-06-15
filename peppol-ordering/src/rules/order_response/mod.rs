// Peppol BIS Ordering 3.0 — OrderResponse Validation Rules
//
// Validates OrderResponse documents according to Peppol BIS Ordering.
// Rules ORESP-R001 through ORESP-R011.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

/// Returns true if the OrderResponseCode value indicates acceptance.
fn is_acceptance(code: &str) -> bool {
    matches!(code, "1" | "CA" | "6" | "accepted" | "Accepted")
        || code.to_lowercase().contains("accept")
}

/// Returns true if the OrderResponseCode value indicates rejection or change.
fn is_rejection_or_change(code: &str) -> bool {
    matches!(
        code,
        "2" | "RE" | "3" | "CH" | "rejected" | "Rejected" | "changed" | "Changed"
    ) || code.to_lowercase().contains("reject")
        || code.to_lowercase().contains("change")
}

mod oresp_r001;
mod oresp_r002;
mod oresp_r003;
mod oresp_r004;
mod oresp_r005;
mod oresp_r006;
mod oresp_r007;
mod oresp_r008;
mod oresp_r009;
mod oresp_r010;
mod oresp_r011;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<OrderResponse>) {
    engine.add_rule(oresp_r001::rule(inv));
    engine.add_rule(oresp_r002::rule(inv));
    engine.add_rule(oresp_r003::rule(inv));
    engine.add_rule(oresp_r004::rule(inv));
    engine.add_rule(oresp_r005::rule(inv));
    engine.add_rule(oresp_r006::rule(inv));
    engine.add_rule(oresp_r007::rule(inv));
    engine.add_rule(oresp_r008::rule(inv));
    engine.add_rule(oresp_r009::rule(inv));
    engine.add_rule(oresp_r010::rule(inv));
    engine.add_rule(oresp_r011::rule(inv));
}
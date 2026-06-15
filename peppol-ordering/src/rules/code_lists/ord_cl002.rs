use peppol_common::codes::payment_means_codes;
/// ORD-CL002 (Fatal): PaymentMeansCode must be from UNCL 4461
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

fn payment_is_a_valid_uncl_4461_code(order: &Arc<Order>) -> Result<(), String> {
    for (i, pm) in order.payment_means.iter().enumerate() {
        let code = pm.payment_means_code.value();
        if !payment_means_codes().is_valid(code) {
            return Err(format!(
                "PaymentMeans[{}] code '{}' is not a valid UNCL4461 code",
                i + 1,
                code
            ));
        }
    }
    Ok(())
}

pub fn rule(order: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-CL002".into(),
        description: "PaymentMeansCode must be a valid UNCL 4461 code".into(),
        severity: Severity::Fatal,
        check: {
            let order = Arc::clone(order);
            Box::new(move || payment_is_a_valid_uncl_4461_code(&order))
        },
    }
}

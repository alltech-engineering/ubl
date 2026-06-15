/// ORD-R043 (Warning): PaymentMeans should include financial account details
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R043".into(),
        description: "PaymentMeans should include financial account details".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    if pm.payee_financial_account.is_none()
                        && pm.payer_financial_account.is_none()
                    {
                        return Err(format!(
                            "PaymentMeans[{}] has no financial account details — consider providing account information",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}

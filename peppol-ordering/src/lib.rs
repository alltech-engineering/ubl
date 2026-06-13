pub mod rules;

use peppol_common::identity::identities;
use peppol_common::identity::{self, BisDocument, DocumentIdentity};
use ubl_documents::ordering::Order;

pub struct PeppolOrder {
    pub order: Order,
    identity: DocumentIdentity,
}
impl PeppolOrder {
    pub fn new(order: Order) -> Self {
        Self {
            order,
            identity: identities::ordering_3_0("Order"),
        }
    }
    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        rules::ordering_rules(&self.order).evaluate_all()
    }
}
impl BisDocument for PeppolOrder {
    fn document_type() -> &'static str {
        "Order"
    }
    fn identity(&self) -> &DocumentIdentity {
        &self.identity
    }
    fn validate_peppol(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        self.validate()
    }
}

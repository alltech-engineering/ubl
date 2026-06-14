pub mod rules;

use peppol_common::identity::identities;
use peppol_common::identity::{BisDocument, DocumentIdentity};
use ubl_documents::catalogue::Catalogue;

pub struct PeppolCatalogue {
    pub catalogue: Catalogue,
    identity: DocumentIdentity,
}

impl PeppolCatalogue {
    pub fn new(catalogue: Catalogue) -> Self {
        Self {
            catalogue,
            identity: identities::catalogue_3_0("Catalogue"),
        }
    }

    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        rules::catalogue_rules(&self.catalogue).evaluate_all()
    }
}

impl BisDocument for PeppolCatalogue {
    fn document_type() -> &'static str {
        "Catalogue"
    }

    fn identity(&self) -> &DocumentIdentity {
        &self.identity
    }

    fn validate_peppol(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        self.validate()
    }
}

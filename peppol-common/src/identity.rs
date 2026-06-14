// Peppol Document Identity — the identifiers that declare a UBL document
// as Peppol BIS compliant.
//
// Every Peppol document MUST carry:
//   - CustomizationID: identifies the BIS specification (e.g., Billing 3.0)
//   - ProfileID: identifies the Peppol process (e.g., Billing)
//
// The full document type identifier is a URN:
//   urn:oasis:names:specification:ubl:schema:xsd:Invoice-2::Invoice##
//   urn:cen.eu:en16931:2017#compliant#
//   urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1

use serde::{Deserialize, Serialize};

/// A Peppol document identity — the CustomizationID and ProfileID that
/// declare a UBL document as compliant with a specific BIS.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentIdentity {
    /// The BIS customization identifier.
    /// e.g., "urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1"
    pub customization_id: String,
    /// The Peppol process/profile identifier.
    /// e.g., "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0"
    pub profile_id: String,
}

/// Trait for types that represent a Peppol BIS document.
/// Implemented by document-specific wrappers (e.g., PeppolInvoice).
pub trait BisDocument {
    /// The UBL document type this wraps (e.g., Invoice, CreditNote).
    fn document_type() -> &'static str;

    /// The document identity for this BIS.
    fn identity(&self) -> &DocumentIdentity;

    /// Validate the document against all Peppol rules.
    fn validate_peppol(&self) -> Vec<crate::rules::RuleOutcome>;
}

/// Pre-defined document identities for common BIS specifications.
pub mod identities {
    use super::DocumentIdentity;

    /// Peppol BIS Billing 3.0 — Invoice
    pub const BILLING_3_0_INVOICE: DocumentIdentity = DocumentIdentity {
        customization_id: String::new(), // filled at construction
        profile_id: String::new(),
    };

    /// Build a Billing 3.0 identity for a specific document type.
    pub fn billing_3_0(doc_type: &str) -> DocumentIdentity {
        DocumentIdentity {
            customization_id: format!("urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1"),
            profile_id: format!("urn:fdc:peppol.eu:2017:poacc:billing:01:1.0"),
        }
    }

    /// Build an Ordering 3.0 identity for a specific document type.
    pub fn ordering_3_0(doc_type: &str) -> DocumentIdentity {
        DocumentIdentity {
            customization_id: format!("urn:fdc:peppol.eu:2017:poacc:ordering:3.0::2.1"),
            profile_id: format!("urn:fdc:peppol.eu:2017:poacc:ordering:01:1.0"),
        }
    }

    /// Build a Despatch 3.0 identity for a specific document type.
    pub fn despatch_3_0(doc_type: &str) -> DocumentIdentity {
        DocumentIdentity {
            customization_id: format!("urn:fdc:peppol.eu:2017:poacc:despatch:3.0::2.1"),
            profile_id: format!("urn:fdc:peppol.eu:2017:poacc:despatch:01:1.0"),
        }
    }

    /// Build a Catalogue 3.0 identity for a specific document type.
    pub fn catalogue_3_0(doc_type: &str) -> DocumentIdentity {
        DocumentIdentity {
            customization_id: format!("urn:fdc:peppol.eu:2017:poacc:catalogue:3.0::2.1"),
            profile_id: format!("urn:fdc:peppol.eu:2017:poacc:catalogue:01:1.0"),
        }
    }

    /// Build an MLR 3.0 identity.
    pub fn mlr_3_0() -> DocumentIdentity {
        DocumentIdentity {
            customization_id: "urn:fdc:peppol.eu:2017:poacc:mlr:3.0::2.1".into(),
            profile_id: "urn:fdc:peppol.eu:2017:poacc:mlr:01:1.0".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_identity() {
        let id = identities::billing_3_0("Invoice");
        assert!(id.customization_id.contains("billing:3.0"));
        assert!(id.profile_id.contains("billing:01"));
    }
}

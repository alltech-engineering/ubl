// Peppol BIS Despatch Advice 3.x
//
// Implements Peppol BIS Despatch Advice validation on top of UBL.
// Provides business rules that check shipment and delivery data.
//
// Reference: https://docs.peppol.eu/poacc/despatch/

pub mod rules;

use ubl_documents::despatch::DespatchAdvice;

/// A Peppol BIS Despatch Advice wrapper.
///
/// Carries the UBL DespatchAdvice plus Peppol-specific validation.
pub struct PeppolDespatch {
    /// The underlying UBL DespatchAdvice
    pub despatch: DespatchAdvice,
}

impl PeppolDespatch {
    /// Create a new Peppol Despatch Advice.
    pub fn new(despatch: DespatchAdvice) -> Self {
        Self { despatch }
    }

    /// Validate this despatch advice against all Peppol Despatch rules.
    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        rules::despatch_rules(&self.despatch).evaluate_all()
    }
}

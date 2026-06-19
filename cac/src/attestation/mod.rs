use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Attestation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "AcceptanceIndicator")]
    pub acceptance_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
    #[serde(default, rename = "AttestationLine")]
    pub attestation_line: Vec<AttestationLine>,
}

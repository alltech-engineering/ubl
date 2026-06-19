use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class describing an attestation made for an item
///
/// UBL Dictionary Entry Name: `Attestation. Details`
///
/// Generated from XSD type `AttestationType`.
pub struct Attestation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this attestation.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A name of this attestation.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A textual description of this attestation.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Indicates whether the attestation has been accepted or not.
    #[serde(default, rename = "AcceptanceIndicator")]
    pub acceptance_indicator: Option<udt::Indicator>,
/// The period during which this attestation is valid
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
/// The Party who issues this Attestation
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
/// An attestation or statement made and which forms part of this attestation
    #[serde(default, rename = "AttestationLine")]
    pub attestation_line: Vec<AttestationLine>,
}

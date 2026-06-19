use serde::{Deserialize, Serialize};


include!("supplied.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an item of evidentiary support for representations of capabilities or the
/// ability to meet tendering requirements, which an economic operator must provide for acceptance into
/// a tendering process.
///
/// UBL Dictionary Entry Name: `Evidence. Details`
///
/// Generated from XSD type `EvidenceType`.
pub struct Evidence {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this item of evidentiary support.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of evidence.
    #[serde(default, rename = "EvidenceTypeCode")]
    pub evidence_type_code: Option<cct::Code>,
/// The name of the evidence.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The textual description for this Evidence.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Information about a candidate statement that the contracting authority accepts as a sufficient
/// response.
    #[serde(default, rename = "CandidateStatement")]
    pub candidate_statement: Vec<cct::Text>,
/// A code specifying the confidentiality level of this evidence.
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<cct::Code>,
/// The Party who issues the evidentiary Document.
    #[serde(default, rename = "EvidenceIssuingParty")]
    pub evidence_issuing_party: Option<crate::Party>,
/// A reference to the evidentiary document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// Information about a required translation to be part of the response, i.e. the language.
    #[serde(default, rename = "Language")]
    pub language: Option<crate::Language>,
}

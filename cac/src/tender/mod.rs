use serde::{Deserialize, Serialize};

pub type TenderEvent = crate::Event;

include!("line.rs");
include!("result.rs");
include!("requirement.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe directions for preparing a tender.
///
/// UBL Dictionary Entry Name: `Tender Preparation. Details`
///
/// Generated from XSD type `TenderPreparationType`.
pub struct TenderPreparation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the tender envelope to be used with the tender.
    #[serde(rename = "TenderEnvelopeID")]
    pub tender_envelope_id: cct::Identifier,
/// A code signifying the type of tender envelope (economical or objective criteria versus technical or
/// subjective criteria).
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: Option<cct::Code>,
/// Text describing the tender envelope.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for the open tender associated with this tender preparation.
    #[serde(default, rename = "OpenTenderID")]
    pub open_tender_id: Option<cct::Identifier>,
/// The procurement project lot associated with a particular tenderer.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<crate::ProcurementProjectLot>,
/// A reference to the template for a required document in a tendering process.
    #[serde(default, rename = "DocumentTenderRequirement")]
    pub document_tender_requirement: Vec<TenderRequirement>,
/// A reference to the details of the encryption process used for the tender.
    #[serde(default, rename = "TenderEncryptionData")]
    pub tender_encryption_data: Vec<crate::EncryptionData>,
}

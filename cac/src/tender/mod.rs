use serde::{Deserialize, Serialize};

pub type TenderEvent = crate::Event;

include!("line.rs");
include!("result.rs");
include!("requirement.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderPreparation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "TenderEnvelopeID")]
    pub tender_envelope_id: cct::Identifier,
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "OpenTenderID")]
    pub open_tender_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<crate::ProcurementProjectLot>,
    #[serde(default, rename = "DocumentTenderRequirement")]
    pub document_tender_requirement: Vec<TenderRequirement>,
    #[serde(default, rename = "TenderEncryptionData")]
    pub tender_encryption_data: Vec<crate::EncryptionData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderPreparation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TenderEnvelopeID")]
    pub tender_envelope_id: super::cct::IdentifierType,
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "OpenTenderID")]
    pub open_tender_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, rename = "DocumentTenderRequirement")]
    pub document_tender_requirement: Vec<TenderRequirement>,
    #[serde(default, rename = "TenderEncryptionData")]
    pub tender_encryption_data: Vec<EncryptionData>,
}

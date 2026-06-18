#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectLot {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: Option<TenderingTerms>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<TenderingProcess>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<ProcurementProject>,
}

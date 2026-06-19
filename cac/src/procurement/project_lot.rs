#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectLot {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: Option<crate::TenderingTerms>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<crate::TenderingProcess>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<ProcurementProject>,
}

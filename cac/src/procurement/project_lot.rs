#[derive(Debug, Deserialize, Serialize)]
/// A class to describe one of the parts of a procurement project that is being subdivided to allow the
/// contracting party to award different lots to different economic operators under different contracts.
///
/// UBL Dictionary Entry Name: `Procurement Project Lot. Details`
///
/// Generated from XSD type `ProcurementProjectLotType`.
pub struct ProcurementProjectLot {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this procurement project lot.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A reference to a legal document.
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: Vec<crate::DocumentReference>,
/// A reference to a technical document.
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: Vec<crate::DocumentReference>,
/// A reference to a required document.
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: Vec<crate::DocumentReference>,
/// A reference to a provided document.
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: Vec<crate::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<crate::DocumentReference>,
/// Tendering terms for this procurement project lot.
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: Option<crate::TenderingTerms>,
/// Tendering process for this procurement project lot.
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<crate::TenderingProcess>,
/// A description of the procurement project to be divided.
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<ProcurementProject>,
}

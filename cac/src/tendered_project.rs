#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a tendered project or project lot.
///
/// UBL Dictionary Entry Name: `Tendered Project. Details`
///
/// Generated from XSD type `TenderedProjectType`.
pub struct TenderedProject {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this variant of a tendered project.
    #[serde(default, rename = "VariantID")]
    pub variant_id: Option<cct::Identifier>,
/// The fee amount for tendered projects.
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: Option<cct::Amount>,
/// Text describing the fee amount for tendered projects.
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
/// An identifier for the tender envelope this tendered project belongs to.
    #[serde(default, rename = "TenderEnvelopeID")]
    pub tender_envelope_id: Option<cct::Identifier>,
/// A code signifying the type of tender envelope this tendered project belongs to.
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: Option<cct::Code>,
/// An additional fee for this tendered project.
    #[serde(default, rename = "AdditionalFee")]
    pub additional_fee: Vec<Fee>,
/// The procurement project lot to which this Tender Line refers to. If there are no lots, this ought
/// not be defined.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
/// A reference to a non-structured evidentiary document supporting this tendered project.
    #[serde(default, rename = "EvidenceDocumentReference")]
    pub evidence_document_reference: Vec<DocumentReference>,
/// A total amount of taxes of a particular kind applicable to the monetary total for this tendered
/// project.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
/// The total amount for this tendered project.
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: Option<MonetaryTotal>,
/// A line in the tender for this tendered project.
    #[serde(default, rename = "TenderLine")]
    pub tender_line: Vec<TenderLine>,
/// An association to an Awarding Criterion Response.
    #[serde(default, rename = "AwardingCriterionResponse")]
    pub awarding_criterion_response: Vec<AwardingCriterionResponse>,
}

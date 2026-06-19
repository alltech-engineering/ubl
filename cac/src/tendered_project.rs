#[derive(Debug, Deserialize, Serialize)]
pub struct TenderedProject {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "VariantID")]
    pub variant_id: Option<cct::Identifier>,
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: Option<cct::Amount>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
    #[serde(default, rename = "TenderEnvelopeID")]
    pub tender_envelope_id: Option<cct::Identifier>,
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: Option<cct::Code>,
    #[serde(default, rename = "AdditionalFee")]
    pub additional_fee: Vec<Fee>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(default, rename = "EvidenceDocumentReference")]
    pub evidence_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: Option<MonetaryTotal>,
    #[serde(default, rename = "TenderLine")]
    pub tender_line: Vec<TenderLine>,
    #[serde(default, rename = "AwardingCriterionResponse")]
    pub awarding_criterion_response: Vec<AwardingCriterionResponse>,
}

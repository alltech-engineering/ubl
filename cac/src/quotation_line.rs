#[derive(Debug, Deserialize, Serialize)]
pub struct QuotationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
    #[serde(default, rename = "RequestForQuotationLineID")]
    pub request_for_quotation_line_id: Option<cct::Identifier>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItem,
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: Vec<LineItem>,
    #[serde(default, rename = "AlternativeLineItem")]
    pub alternative_line_item: Vec<LineItem>,
    #[serde(default, rename = "RequestLineReference")]
    pub request_line_reference: Option<LineReference>,
}

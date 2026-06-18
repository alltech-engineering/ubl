#[derive(Debug, Deserialize, Serialize)]
pub struct QuotationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "RequestForQuotationLineID")]
    pub request_for_quotation_line_id: Option<super::cct::IdentifierType>,
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

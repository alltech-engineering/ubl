#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SubstitutionStatusCode")]
    pub substitution_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItem,
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: Vec<LineItem>,
    #[serde(default, rename = "SellerSubstitutedLineItem")]
    pub seller_substituted_line_item: Vec<LineItem>,
    #[serde(default, rename = "BuyerProposedSubstituteLineItem")]
    pub buyer_proposed_substitute_line_item: Vec<LineItem>,
    #[serde(default, rename = "CatalogueLineReference")]
    pub catalogue_line_reference: Option<LineReference>,
    #[serde(default, rename = "QuotationLineReference")]
    pub quotation_line_reference: Option<LineReference>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}

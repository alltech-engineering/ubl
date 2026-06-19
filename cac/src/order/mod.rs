use serde::{Deserialize, Serialize};

pub type OrderDocumentReference = crate::DocumentReference;

include!("reference.rs");
include!("line_reference.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SubstitutionStatusCode")]
    pub substitution_status_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "LineItem")]
    pub line_item: crate::LineItem,
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: Vec<crate::LineItem>,
    #[serde(default, rename = "SellerSubstitutedLineItem")]
    pub seller_substituted_line_item: Vec<crate::LineItem>,
    #[serde(default, rename = "BuyerProposedSubstituteLineItem")]
    pub buyer_proposed_substitute_line_item: Vec<crate::LineItem>,
    #[serde(default, rename = "CatalogueLineReference")]
    pub catalogue_line_reference: Option<crate::LineReference>,
    #[serde(default, rename = "QuotationLineReference")]
    pub quotation_line_reference: Option<crate::LineReference>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
}

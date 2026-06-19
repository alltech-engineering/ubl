use serde::{Deserialize, Serialize};

pub type RequestReceptionPeriod = crate::Period;
pub type RequestRecipientParty = crate::Party;

include!("for_quotation_line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForTenderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
    #[serde(default, rename = "EstimatedAmount")]
    pub estimated_amount: Option<cct::Amount>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "DeliveryPeriod")]
    pub delivery_period: Vec<crate::Period>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
    #[serde(default, rename = "SubRequestForTenderLine")]
    pub sub_request_for_tender_line: Vec<RequestForTenderLine>,
}

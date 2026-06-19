use serde::{Deserialize, Serialize};

pub type RequestReceptionPeriod = crate::Period;
pub type RequestRecipientParty = crate::Party;

include!("for_quotation_line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Request for Tender describing an item of goods or a service solicited
/// in the Request for Tender.
///
/// UBL Dictionary Entry Name: `Request For Tender Line. Details`
///
/// Generated from XSD type `RequestForTenderLineType`.
pub struct RequestForTenderLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this request for tender line.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A universally unique identifier for this request for tender line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of the item for which a tender is requested in this line.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The minimum quantity of the item associated with this request for tender line.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The maximum quantity of the item associated with this request for tender line.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// Indicates whether the amounts are taxes included (true) or not (false).
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
/// The minimum amount allowed for this deliverable.
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
/// The maximum amount allowed for this deliverable.
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
/// The estimated total amount of the deliverable.
    #[serde(default, rename = "EstimatedAmount")]
    pub estimated_amount: Option<cct::Amount>,
/// A reference to a document associated with this request for tender line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// An applicable period for the deliverable or set of deliverables in this tendering process.
    #[serde(default, rename = "DeliveryPeriod")]
    pub delivery_period: Vec<crate::Period>,
/// Properties of the item specified in this request for tender line that are dependent on location and
/// quantity.
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
/// The period during which a warranty to be associated with this request for tender line must apply.
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
/// An item for which a tender is requested.
    #[serde(rename = "Item")]
    pub item: crate::Item,
/// A subsidiary request for tender line.
    #[serde(default, rename = "SubRequestForTenderLine")]
    pub sub_request_for_tender_line: Vec<RequestForTenderLine>,
}

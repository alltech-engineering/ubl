use serde::{Deserialize, Serialize};

pub type OrderDocumentReference = crate::DocumentReference;

include!("reference.rs");
include!("line_reference.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an order document (e.g., Order, Order Change, or Order Response)
/// describing an item being ordered.
///
/// UBL Dictionary Entry Name: `Order Line. Details`
///
/// Generated from XSD type `OrderLineType`.
pub struct OrderLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the substitution status of the item on this order line. The order line may
/// indicate that the substitute is proposed by the buyer (in Order) or by the seller (in Order
/// Response) or that a substitution has been made by the seller (in Order Response).
    #[serde(default, rename = "SubstitutionStatusCode")]
    pub substitution_status_code: Option<cct::Code>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The line item itself.
    #[serde(rename = "LineItem")]
    pub line_item: crate::LineItem,
/// In Order Response, a line item proposed by the seller describing a product that might substitute for
/// the product described in this order line.
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: Vec<crate::LineItem>,
/// In Order Response, a line item that has replaced the original order line item. The specified
/// quantity and pricing may differ from those in the original line item, but when a line item is
/// substituted by the seller, it is assumed that other information, such as shipment details, will
/// remain the same.
    #[serde(default, rename = "SellerSubstitutedLineItem")]
    pub seller_substituted_line_item: Vec<crate::LineItem>,
/// A description of an item proposed by the buyer as a possible alternative to the item associated with
/// this order line.
    #[serde(default, rename = "BuyerProposedSubstituteLineItem")]
    pub buyer_proposed_substitute_line_item: Vec<crate::LineItem>,
/// A reference to a catalogue line associated with this order line.
    #[serde(default, rename = "CatalogueLineReference")]
    pub catalogue_line_reference: Option<crate::LineReference>,
/// A reference to a quotation line associated with this order line.
    #[serde(default, rename = "QuotationLineReference")]
    pub quotation_line_reference: Option<crate::LineReference>,
/// A reference to another order line, such as in a replacement order or another line on the same order
/// that is related.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
/// A reference to a document associated with this order line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
}

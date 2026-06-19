#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a line item associated with a promotional event.
///
/// UBL Dictionary Entry Name: `Promotional Event Line Item. Details`
///
/// Generated from XSD type `PromotionalEventLineItemType`.
pub struct PromotionalEventLineItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The amount associated with this promotional event line item.
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
/// A line item describing the expected impacts associated with this promotional event for a specific
/// product at a specific location.
    #[serde(rename = "EventLineItem")]
    pub event_line_item: crate::EventLineItem,
}

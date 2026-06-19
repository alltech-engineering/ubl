#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a promotional event as a set of item locations that share a set of promotional
/// tactics.
///
/// UBL Dictionary Entry Name: `Promotional Specification. Details`
///
/// Generated from XSD type `PromotionalSpecificationType`.
pub struct PromotionalSpecification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this promotional specification.
    #[serde(default, rename = "SpecificationID")]
    pub specification_id: Option<cct::Identifier>,
/// A line item for a promotional event involving a specific product at a specific location; it
/// describes the expected impacts associated with the event and specifies the promotional price of the
/// item."
    #[serde(default, rename = "PromotionalEventLineItem")]
    pub promotional_event_line_item: Vec<PromotionalEventLineItem>,
/// An event tactic associated with this promotion.
    #[serde(default, rename = "EventTactic")]
    pub event_tactic: Vec<crate::EventTactic>,
}

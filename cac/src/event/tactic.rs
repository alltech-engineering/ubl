#[derive(Debug, Deserialize, Serialize)]
/// A class defining a specific type of action or situation arranged by the Buyer or the Seller to
/// promote the product or products.
///
/// UBL Dictionary Entry Name: `Event Tactic. Details`
///
/// Generated from XSD type `EventTacticType`.
pub struct EventTactic {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// Generic field to add additional information or to specify mutually defined eventTacticTypes that are
/// not currently listed.
    #[serde(default, rename = "Comment")]
    pub comment: Option<cct::Text>,
/// The currencies, units, etc. that describes what is need for the event or promotion Usage example:
/// Number of pallets per store for a stack display
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The set of codes that describes this event tactic.
    #[serde(rename = "EventTacticEnumeration")]
    pub event_tactic_enumeration: EventTacticEnumeration,
/// The period covered by this event tactic.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
}

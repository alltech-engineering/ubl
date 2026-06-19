#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line item describing the expected impacts associated with a retail event
/// involving a specific product at a specific location.
///
/// UBL Dictionary Entry Name: `Event Line Item. Details`
///
/// Generated from XSD type `EventLineItemType`.
pub struct EventLineItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The number of this event line item.
    #[serde(default, rename = "LineNumberNumeric")]
    pub line_number_numeric: Option<cct::Numeric>,
/// The location of the stores involved in the event described in this line item.
    #[serde(default, rename = "ParticipatingLocationsLocation")]
    pub participating_locations_location: Option<crate::Location>,
/// A planned impact of the event described in this line item.
    #[serde(default, rename = "RetailPlannedImpact")]
    pub retail_planned_impact: Vec<crate::RetailPlannedImpact>,
/// The product with which the event is associated.
    #[serde(rename = "SupplyItem")]
    pub supply_item: crate::Item,
}

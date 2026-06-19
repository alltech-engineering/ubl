#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a miscellaneous event associated with a retail event.
///
/// UBL Dictionary Entry Name: `Miscellaneous Event. Details`
///
/// Generated from XSD type `MiscellaneousEventType`.
pub struct MiscellaneousEvent {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of this miscellaneous event. Examples are: ASSORTMENT_CHARGE DISASTER
/// FORECAST_DECREASE FORECAST_INCREASE FREIGHT_FLOW_ALLOCATION INVENTORY_POLICY_CHANGE LOCATION_CLOSING
/// LOCATION_OPENING OTHER OUT_OF_STOCK PACKAGING_LABELING_CHANGE PRICE_DECREASE PRICE_INCREASE
/// STORE_FORMAT_OR_PLANOGRAM_CHANGE TEST_MARKET WEATHER
    #[serde(rename = "MiscellaneousEventTypeCode")]
    pub miscellaneous_event_type_code: cct::Code,
/// An event line item for this miscellaneous retail event.
    #[serde(default, rename = "EventLineItem")]
    pub event_line_item: Vec<EventLineItem>,
}

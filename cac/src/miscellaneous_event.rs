#[derive(Debug, Deserialize, Serialize)]
pub struct MiscellaneousEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "MiscellaneousEventTypeCode")]
    pub miscellaneous_event_type_code: cct::Code,
    #[serde(default, rename = "EventLineItem")]
    pub event_line_item: Vec<EventLineItem>,
}

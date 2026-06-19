#[derive(Debug, Deserialize, Serialize)]
pub struct MiscellaneousEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "MiscellaneousEventTypeCode")]
    pub miscellaneous_event_type_code: cct::Code,
    #[serde(default, rename = "EventLineItem")]
    pub event_line_item: Vec<EventLineItem>,
}

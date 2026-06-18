#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalEventLineItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(rename = "EventLineItem")]
    pub event_line_item: EventLineItem,
}

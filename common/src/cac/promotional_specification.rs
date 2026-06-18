#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalSpecification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SpecificationID")]
    pub specification_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PromotionalEventLineItem")]
    pub promotional_event_line_item: Vec<PromotionalEventLineItem>,
    #[serde(default, rename = "EventTactic")]
    pub event_tactic: Vec<EventTactic>,
}

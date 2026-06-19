#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalSpecification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "SpecificationID")]
    pub specification_id: Option<cct::Identifier>,
    #[serde(default, rename = "PromotionalEventLineItem")]
    pub promotional_event_line_item: Vec<PromotionalEventLineItem>,
    #[serde(default, rename = "EventTactic")]
    pub event_tactic: Vec<crate::EventTactic>,
}

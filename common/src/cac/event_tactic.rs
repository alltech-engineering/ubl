#[derive(Debug, Deserialize, Serialize)]
pub struct EventTactic {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Comment")]
    pub comment: Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(rename = "EventTacticEnumeration")]
    pub event_tactic_enumeration: EventTacticEnumeration,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}

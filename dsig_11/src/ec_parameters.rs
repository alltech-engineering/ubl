#[derive(Debug, Deserialize, Serialize)]
pub struct EcParameters {
    #[serde(rename = "FieldID")]
    pub field_id: FieldId,
    #[serde(rename = "Curve")]
    pub curve: Curve,
    #[serde(rename = "Base")]
    pub base: String,
    #[serde(rename = "Order")]
    pub order: String,
    #[serde(default, rename = "CoFactor")]
    pub co_factor: Option<i32>,
    #[serde(default, rename = "ValidationData")]
    pub validation_data: Option<EcValidationData>,
}

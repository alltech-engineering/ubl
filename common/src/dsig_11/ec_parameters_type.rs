#[derive(Debug, Deserialize, Serialize)]
pub struct EcParametersType {
    #[serde(rename = "FieldID")]
    pub field_id: FieldIdType,
    #[serde(rename = "Curve")]
    pub curve: CurveType,
    #[serde(rename = "Base")]
    pub base: String,
    #[serde(rename = "Order")]
    pub order: String,
    #[serde(default, rename = "CoFactor")]
    pub co_factor: Option<i32>,
    #[serde(default, rename = "ValidationData")]
    pub validation_data: Option<EcValidationDataType>,
}

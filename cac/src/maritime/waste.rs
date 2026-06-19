#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeWaste {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: Option<cct::Code>,
    #[serde(default, rename = "ToBeDeliveredMeasure")]
    pub to_be_delivered_measure: Option<cct::Measure>,
    #[serde(default, rename = "RetainedOnBoardMeasure")]
    pub retained_on_board_measure: Option<cct::Measure>,
    #[serde(default, rename = "MaxDedicatedStorageCapacityMeasure")]
    pub max_dedicated_storage_capacity_measure: Option<cct::Measure>,
    #[serde(default, rename = "EstimatedGeneratedUntilNextPortMeasure")]
    pub estimated_generated_until_next_port_measure: Option<cct::Measure>,
    #[serde(default, rename = "RemainingWasteDeliveryPortLocation")]
    pub remaining_waste_delivery_port_location: Vec<crate::Location>,
}

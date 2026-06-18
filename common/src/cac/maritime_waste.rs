#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeWaste {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ToBeDeliveredMeasure")]
    pub to_be_delivered_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "RetainedOnBoardMeasure")]
    pub retained_on_board_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaxDedicatedStorageCapacityMeasure")]
    pub max_dedicated_storage_capacity_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "EstimatedGeneratedUntilNextPortMeasure")]
    pub estimated_generated_until_next_port_measure:
        Option<super::cct::MeasureType>,
    #[serde(default, rename = "RemainingWasteDeliveryPortLocation")]
    pub remaining_waste_delivery_port_location: Vec<Location>,
}

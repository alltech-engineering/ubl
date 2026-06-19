#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a transaction of maritime waste.
///
/// UBL Dictionary Entry Name: `Maritime Waste. Details`
///
/// Generated from XSD type `MaritimeWasteType`.
pub struct MaritimeWaste {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this maritime waste transaction.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A text descriping this maritime waste transaction.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A code specifying the type of waste in this maritime waste transaction.
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: Option<cct::Code>,
/// The messure of waste to be delivered.
    #[serde(default, rename = "ToBeDeliveredMeasure")]
    pub to_be_delivered_measure: Option<cct::Measure>,
/// The meassure of waste retained on board.
    #[serde(default, rename = "RetainedOnBoardMeasure")]
    pub retained_on_board_measure: Option<cct::Measure>,
/// The messure for the maximum dedicated storage capacity.
    #[serde(default, rename = "MaxDedicatedStorageCapacityMeasure")]
    pub max_dedicated_storage_capacity_measure: Option<cct::Measure>,
/// The messure of waste generated until the next port.
    #[serde(default, rename = "EstimatedGeneratedUntilNextPortMeasure")]
    pub estimated_generated_until_next_port_measure: Option<cct::Measure>,
/// The location of the port where the remaining waste is delivered.
    #[serde(default, rename = "RemainingWasteDeliveryPortLocation")]
    pub remaining_waste_delivery_port_location: Vec<crate::Location>,
}

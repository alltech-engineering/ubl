#[derive(Debug, Deserialize, Serialize)]
pub struct EventLineItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LineNumberNumeric")]
    pub line_number_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ParticipatingLocationsLocation")]
    pub participating_locations_location: Option<Location>,
    #[serde(default, rename = "RetailPlannedImpact")]
    pub retail_planned_impact: Vec<RetailPlannedImpact>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: Item,
}

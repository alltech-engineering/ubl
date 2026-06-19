#[derive(Debug, Deserialize, Serialize)]
pub struct EventLineItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LineNumberNumeric")]
    pub line_number_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ParticipatingLocationsLocation")]
    pub participating_locations_location: Option<crate::Location>,
    #[serde(default, rename = "RetailPlannedImpact")]
    pub retail_planned_impact: Vec<crate::RetailPlannedImpact>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: crate::Item,
}

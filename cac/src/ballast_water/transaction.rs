#[derive(Debug, Deserialize, Serialize)]
pub struct BallastWaterTransaction {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TankID")]
    pub tank_id: Option<cct::Identifier>,
    #[serde(default, rename = "TankTypeCode")]
    pub tank_type_code: Option<cct::Code>,
    #[serde(default, rename = "ExchangeMethodCode")]
    pub exchange_method_code: Option<cct::Code>,
    #[serde(default, rename = "ExchangedPercent")]
    pub exchanged_percent: Option<cct::Numeric>,
    #[serde(default, rename = "VolumeMeasure")]
    pub volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "SeaHeightMeasure")]
    pub sea_height_measure: Option<cct::Measure>,
    #[serde(default, rename = "SalinityMeasure")]
    pub salinity_measure: Option<cct::Measure>,
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: Option<udt::DateTime>,
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
    #[serde(default, rename = "BallastWaterTemperature")]
    pub ballast_water_temperature: Option<crate::Temperature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BallastWaterTransaction {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TankID")]
    pub tank_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TankTypeCode")]
    pub tank_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangeMethodCode")]
    pub exchange_method_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangedPercent")]
    pub exchanged_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "VolumeMeasure")]
    pub volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "SeaHeightMeasure")]
    pub sea_height_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "SalinityMeasure")]
    pub salinity_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Location")]
    pub location: Option<Location>,
    #[serde(default, rename = "BallastWaterTemperature")]
    pub ballast_water_temperature: Option<Temperature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeterReading {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "MeterReadingType")]
    pub meter_reading_type: Option<cct::Text>,
    #[serde(default, rename = "MeterReadingTypeCode")]
    pub meter_reading_type_code: Option<cct::Code>,
    #[serde(rename = "PreviousMeterReadingDate")]
    pub previous_meter_reading_date: udt::DateTime,
    #[serde(rename = "PreviousMeterQuantity")]
    pub previous_meter_quantity: cct::Quantity,
    #[serde(rename = "LatestMeterReadingDate")]
    pub latest_meter_reading_date: udt::DateTime,
    #[serde(rename = "LatestMeterQuantity")]
    pub latest_meter_quantity: cct::Quantity,
    #[serde(default, rename = "PreviousMeterReadingMethod")]
    pub previous_meter_reading_method: Option<cct::Text>,
    #[serde(default, rename = "PreviousMeterReadingMethodCode")]
    pub previous_meter_reading_method_code: Option<cct::Code>,
    #[serde(default, rename = "LatestMeterReadingMethod")]
    pub latest_meter_reading_method: Option<cct::Text>,
    #[serde(default, rename = "LatestMeterReadingMethodCode")]
    pub latest_meter_reading_method_code: Option<cct::Code>,
    #[serde(default, rename = "MeterReadingComments")]
    pub meter_reading_comments: Vec<cct::Text>,
    #[serde(rename = "DeliveredQuantity")]
    pub delivered_quantity: cct::Quantity,
}

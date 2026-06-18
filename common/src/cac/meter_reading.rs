#[derive(Debug, Deserialize, Serialize)]
pub struct MeterReading {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MeterReadingType")]
    pub meter_reading_type: Option<super::cct::TextType>,
    #[serde(default, rename = "MeterReadingTypeCode")]
    pub meter_reading_type_code: Option<super::cct::CodeType>,
    #[serde(rename = "PreviousMeterReadingDate")]
    pub previous_meter_reading_date: super::udt::DateTimeType,
    #[serde(rename = "PreviousMeterQuantity")]
    pub previous_meter_quantity: super::cct::QuantityType,
    #[serde(rename = "LatestMeterReadingDate")]
    pub latest_meter_reading_date: super::udt::DateTimeType,
    #[serde(rename = "LatestMeterQuantity")]
    pub latest_meter_quantity: super::cct::QuantityType,
    #[serde(default, rename = "PreviousMeterReadingMethod")]
    pub previous_meter_reading_method: Option<super::cct::TextType>,
    #[serde(default, rename = "PreviousMeterReadingMethodCode")]
    pub previous_meter_reading_method_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LatestMeterReadingMethod")]
    pub latest_meter_reading_method: Option<super::cct::TextType>,
    #[serde(default, rename = "LatestMeterReadingMethodCode")]
    pub latest_meter_reading_method_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MeterReadingComments")]
    pub meter_reading_comments: Vec<super::cct::TextType>,
    #[serde(rename = "DeliveredQuantity")]
    pub delivered_quantity: super::cct::QuantityType,
}

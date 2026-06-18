#[derive(Debug, Deserialize, Serialize)]
pub struct Meter {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<super::cct::TextType>,
    #[serde(default, rename = "MeterName")]
    pub meter_name: Option<super::cct::TextType>,
    #[serde(default, rename = "MeterConstant")]
    pub meter_constant: Option<super::cct::TextType>,
    #[serde(default, rename = "MeterConstantCode")]
    pub meter_constant_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MeterReading")]
    pub meter_reading: Vec<MeterReading>,
    #[serde(default, rename = "MeterProperty")]
    pub meter_property: Vec<MeterProperty>,
}

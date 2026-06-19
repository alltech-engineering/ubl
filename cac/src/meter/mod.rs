use serde::{Deserialize, Serialize};


include!("reading.rs");
include!("property.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Meter {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
    #[serde(default, rename = "MeterName")]
    pub meter_name: Option<cct::Text>,
    #[serde(default, rename = "MeterConstant")]
    pub meter_constant: Option<cct::Text>,
    #[serde(default, rename = "MeterConstantCode")]
    pub meter_constant_code: Option<cct::Code>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MeterReading")]
    pub meter_reading: Vec<MeterReading>,
    #[serde(default, rename = "MeterProperty")]
    pub meter_property: Vec<MeterProperty>,
}

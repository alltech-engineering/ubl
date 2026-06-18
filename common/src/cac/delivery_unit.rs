#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryUnit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "BatchQuantity")]
    pub batch_quantity: super::cct::QuantityType,
    #[serde(default, rename = "ConsumerUnitQuantity")]
    pub consumer_unit_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
}

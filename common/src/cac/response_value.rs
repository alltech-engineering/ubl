#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseValue {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Response")]
    pub response: Vec<super::cct::TextType>,
    #[serde(default, rename = "ResponseAmount")]
    pub response_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ResponseBinaryObject")]
    pub response_binary_object: Option<super::cct::BinaryObjectType>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ResponseDate")]
    pub response_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ResponseID")]
    pub response_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ResponseIndicator")]
    pub response_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ResponseMeasure")]
    pub response_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "ResponseNumeric")]
    pub response_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ResponseQuantity")]
    pub response_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ResponseTime")]
    pub response_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ResponseURI")]
    pub response_uri: Option<super::cct::IdentifierType>,
}

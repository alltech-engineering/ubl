#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseValue {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Response")]
    pub response: Vec<cct::Text>,
    #[serde(default, rename = "ResponseAmount")]
    pub response_amount: Option<cct::Amount>,
    #[serde(default, rename = "ResponseBinaryObject")]
    pub response_binary_object: Option<cct::BinaryObject>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<cct::Code>,
    #[serde(default, rename = "ResponseDate")]
    pub response_date: Option<udt::DateTime>,
    #[serde(default, rename = "ResponseID")]
    pub response_id: Option<cct::Identifier>,
    #[serde(default, rename = "ResponseIndicator")]
    pub response_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ResponseMeasure")]
    pub response_measure: Option<cct::Measure>,
    #[serde(default, rename = "ResponseNumeric")]
    pub response_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ResponseQuantity")]
    pub response_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ResponseTime")]
    pub response_time: Option<udt::DateTime>,
    #[serde(default, rename = "ResponseURI")]
    pub response_uri: Option<cct::Identifier>,
}

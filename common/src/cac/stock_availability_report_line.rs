#[derive(Debug, Deserialize, Serialize)]
pub struct StockAvailabilityReportLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: Option<super::cct::CodeType>,
    #[serde(rename = "Item")]
    pub item: Item,
}

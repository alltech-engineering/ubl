#[derive(Debug, Deserialize, Serialize)]
pub struct StockAvailabilityReportLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: Option<udt::DateTime>,
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: Option<cct::Code>,
    #[serde(rename = "Item")]
    pub item: Item,
}

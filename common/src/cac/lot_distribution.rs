#[derive(Debug, Deserialize, Serialize)]
pub struct LotDistribution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MaximumLotsAwardedNumeric")]
    pub maximum_lots_awarded_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumLotsSubmittedNumeric")]
    pub maximum_lots_submitted_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "GroupingLots")]
    pub grouping_lots: Vec<super::cct::TextType>,
    #[serde(default, rename = "LotsGroup")]
    pub lots_group: Vec<LotsGroup>,
}

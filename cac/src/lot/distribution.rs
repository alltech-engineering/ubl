#[derive(Debug, Deserialize, Serialize)]
pub struct LotDistribution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "MaximumLotsAwardedNumeric")]
    pub maximum_lots_awarded_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MaximumLotsSubmittedNumeric")]
    pub maximum_lots_submitted_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "GroupingLots")]
    pub grouping_lots: Vec<cct::Text>,
    #[serde(default, rename = "LotsGroup")]
    pub lots_group: Vec<crate::LotsGroup>,
}

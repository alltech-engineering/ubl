#[derive(Debug, Deserialize, Serialize)]
/// A class defining how to treat different lots in a single procurement.
///
/// UBL Dictionary Entry Name: `Lot Distribution. Details`
///
/// Generated from XSD type `LotDistributionType`.
pub struct LotDistribution {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The maximum number of lots that can be awarded to a single tenderer.
    #[serde(default, rename = "MaximumLotsAwardedNumeric")]
    pub maximum_lots_awarded_numeric: Option<cct::Numeric>,
/// The maximum number of lots to which a tenderer can submit an offer to.
    #[serde(default, rename = "MaximumLotsSubmittedNumeric")]
    pub maximum_lots_submitted_numeric: Option<cct::Numeric>,
/// Description on how to combine lots when submitting a tender.
    #[serde(default, rename = "GroupingLots")]
    pub grouping_lots: Vec<cct::Text>,
/// A combination of lots used when evaluating a tender.
    #[serde(default, rename = "LotsGroup")]
    pub lots_group: Vec<crate::LotsGroup>,
}

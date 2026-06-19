#[derive(Debug, Deserialize, Serialize)]
/// A class to describe something valuable offered or striven for in competition.
///
/// UBL Dictionary Entry Name: `Prize. Details`
///
/// Generated from XSD type `PrizeType`.
pub struct Prize {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The relative position in the competition associated with a prize.
    #[serde(rename = "RankCode")]
    pub rank_code: cct::Code,
/// The monetary value amount of a prize.
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
/// Text providing more information about this prize.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}

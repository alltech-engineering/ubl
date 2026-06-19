#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an assigned numeric or qualitative score using a recognized scoring system.
///
/// UBL Dictionary Entry Name: `Score. Details`
///
/// Generated from XSD type `ScoreType`.
pub struct Score {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A numeric value representing the score assigned to this item by a recognized scoring system.
    #[serde(rename = "ScoreNumeric")]
    pub score_numeric: cct::Numeric,
/// A code identifying the scoring system used to determine this Score.
    #[serde(rename = "ScoringSystemCode")]
    pub scoring_system_code: cct::Code,
}

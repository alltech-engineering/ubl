#[derive(Debug, Deserialize, Serialize)]
pub struct Score {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ScoreNumeric")]
    pub score_numeric: cct::Numeric,
    #[serde(rename = "ScoringSystemCode")]
    pub scoring_system_code: cct::Code,
}

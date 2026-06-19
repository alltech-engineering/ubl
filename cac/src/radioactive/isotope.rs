#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveIsotope {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
    #[serde(rename = "ActivityLevelMeasure")]
    pub activity_level_measure: cct::Measure,
}

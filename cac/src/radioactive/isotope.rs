#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveIsotope {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
    #[serde(rename = "ActivityLevelMeasure")]
    pub activity_level_measure: cct::Measure,
}

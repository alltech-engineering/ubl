#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceFrequency {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "WeekDayCode")]
    pub week_day_code: cct::Code,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceFrequency {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "WeekDayCode")]
    pub week_day_code: cct::Code,
}

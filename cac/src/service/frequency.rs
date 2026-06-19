#[derive(Debug, Deserialize, Serialize)]
/// A class to specify which day of the week a transport service is operational.
///
/// UBL Dictionary Entry Name: `Service Frequency. Details`
///
/// Generated from XSD type `ServiceFrequencyType`.
pub struct ServiceFrequency {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A day of the week, expressed as code.
    #[serde(rename = "WeekDayCode")]
    pub week_day_code: cct::Code,
}

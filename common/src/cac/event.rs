#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<Status>,
    #[serde(default, rename = "Contact")]
    pub contact: Vec<Contact>,
    #[serde(default, rename = "OccurenceLocation")]
    pub occurence_location: Option<Location>,
    #[serde(default, rename = "OccurrenceLocation")]
    pub occurrence_location: Option<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallRecord {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SecurityLevelCode")]
    pub security_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SecurityMeasure")]
    pub security_measure: Vec<SecurityMeasure>,
    #[serde(default, rename = "PortFacilityLocation")]
    pub port_facility_location: Option<Location>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}

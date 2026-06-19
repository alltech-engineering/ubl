#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallRecord {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "SecurityLevelCode")]
    pub security_level_code: Option<cct::Code>,
    #[serde(default, rename = "SecurityMeasure")]
    pub security_measure: Vec<crate::SecurityMeasure>,
    #[serde(default, rename = "PortFacilityLocation")]
    pub port_facility_location: Option<crate::Location>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
}

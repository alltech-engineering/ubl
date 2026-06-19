#[derive(Debug, Deserialize, Serialize)]
pub struct ShipToShipActivityRecord {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "AppliedSecurityMeasure")]
    pub applied_security_measure: Vec<crate::SecurityMeasure>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
}

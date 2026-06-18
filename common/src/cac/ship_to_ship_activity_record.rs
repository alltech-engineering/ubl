#[derive(Debug, Deserialize, Serialize)]
pub struct ShipToShipActivityRecord {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "AppliedSecurityMeasure")]
    pub applied_security_measure: Vec<SecurityMeasure>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "Location")]
    pub location: Option<Location>,
}

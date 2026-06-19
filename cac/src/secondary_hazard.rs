#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryHazard {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: Option<cct::Text>,
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: Option<cct::Text>,
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: Option<cct::Code>,
    #[serde(default, rename = "Extension")]
    pub extension: Vec<cct::Text>,
}

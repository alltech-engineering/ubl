#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryHazard {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
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

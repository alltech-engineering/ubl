#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryHazard {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: Option<super::cct::TextType>,
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: Option<super::cct::TextType>,
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Extension")]
    pub extension: Vec<super::cct::TextType>,
}

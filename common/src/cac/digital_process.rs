#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalProcess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DigitalCollaboration")]
    pub digital_collaboration: Vec<DigitalCollaboration>,
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: Vec<DocumentReference>,
}

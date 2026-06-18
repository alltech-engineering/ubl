#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "JobTitle")]
    pub job_title: Option<super::cct::TextType>,
    #[serde(default, rename = "Department")]
    pub department: Option<super::cct::TextType>,
    #[serde(default, rename = "Telephone")]
    pub telephone: Option<super::cct::TextType>,
    #[serde(default, rename = "Telefax")]
    pub telefax: Option<super::cct::TextType>,
    #[serde(default, rename = "ElectronicMail")]
    pub electronic_mail: Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "OtherCommunication")]
    pub other_communication: Vec<Communication>,
}

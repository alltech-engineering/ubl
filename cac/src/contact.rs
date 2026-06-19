#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "JobTitle")]
    pub job_title: Option<cct::Text>,
    #[serde(default, rename = "Department")]
    pub department: Option<cct::Text>,
    #[serde(default, rename = "Telephone")]
    pub telephone: Option<cct::Text>,
    #[serde(default, rename = "Telefax")]
    pub telefax: Option<cct::Text>,
    #[serde(default, rename = "ElectronicMail")]
    pub electronic_mail: Option<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "OtherCommunication")]
    pub other_communication: Vec<Communication>,
}

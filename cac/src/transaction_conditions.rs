#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionConditions {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}

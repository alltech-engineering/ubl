#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialInstitution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Address")]
    pub address: Option<Address>,
}

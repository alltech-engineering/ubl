#[derive(Debug, Deserialize, Serialize)]
pub struct Branch {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "FinancialInstitution")]
    pub financial_institution: Option<FinancialInstitution>,
    #[serde(default, rename = "Address")]
    pub address: Option<Address>,
}

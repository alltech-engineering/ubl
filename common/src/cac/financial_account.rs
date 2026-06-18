#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "AliasName")]
    pub alias_name: Option<super::cct::TextType>,
    #[serde(default, rename = "AccountTypeCode")]
    pub account_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountFormatCode")]
    pub account_format_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "BlockchainID")]
    pub blockchain_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentNote")]
    pub payment_note: Vec<super::cct::TextType>,
    #[serde(default, rename = "FinancialInstitutionBranch")]
    pub financial_institution_branch: Option<Branch>,
    #[serde(default, rename = "Country")]
    pub country: Option<Country>,
}

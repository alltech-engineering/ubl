use serde::{Deserialize, Serialize};


include!("guarantee.rs");
include!("institution.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "AliasName")]
    pub alias_name: Option<cct::Text>,
    #[serde(default, rename = "AccountTypeCode")]
    pub account_type_code: Option<cct::Code>,
    #[serde(default, rename = "AccountFormatCode")]
    pub account_format_code: Option<cct::Code>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<cct::Code>,
    #[serde(default, rename = "BlockchainID")]
    pub blockchain_id: Option<cct::Identifier>,
    #[serde(default, rename = "PaymentNote")]
    pub payment_note: Vec<cct::Text>,
    #[serde(default, rename = "FinancialInstitutionBranch")]
    pub financial_institution_branch: Option<crate::Branch>,
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
}

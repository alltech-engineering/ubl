use serde::{Deserialize, Serialize};


include!("guarantee.rs");
include!("institution.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a financial account.
///
/// UBL Dictionary Entry Name: `Financial Account. Details`
///
/// Generated from XSD type `FinancialAccountType`.
pub struct FinancialAccount {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The identifier for this financial account; the bank account number or wallet identifer.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this financial account.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// An alias for the name of this financial account, to be used in place of the actual account name for
/// security reasons.
    #[serde(default, rename = "AliasName")]
    pub alias_name: Option<cct::Text>,
/// A code signifying the type of this financial account.
    #[serde(default, rename = "AccountTypeCode")]
    pub account_type_code: Option<cct::Code>,
/// A code signifying the format of this financial account.
    #[serde(default, rename = "AccountFormatCode")]
    pub account_format_code: Option<cct::Code>,
/// A code signifying the fiat or crypto currency in which this financial account is held.
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<cct::Code>,
/// An identifier of the Blockchain on which the crypto or stablecoin is being held.
    #[serde(default, rename = "BlockchainID")]
    pub blockchain_id: Option<cct::Identifier>,
/// Free-form text applying to the Payment for the owner of this account.
    #[serde(default, rename = "PaymentNote")]
    pub payment_note: Vec<cct::Text>,
/// The branch of the financial institution associated with this financial account.
    #[serde(default, rename = "FinancialInstitutionBranch")]
    pub financial_institution_branch: Option<crate::Branch>,
/// The country in which the holder of the financial account is domiciled.
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
}

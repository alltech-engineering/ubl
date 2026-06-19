#[derive(Debug, Deserialize, Serialize)]
/// A class to identify a credit account for sales on account.
///
/// UBL Dictionary Entry Name: `Credit Account. Details`
///
/// Generated from XSD type `CreditAccountType`.
pub struct CreditAccount {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this credit account.
    #[serde(rename = "AccountID")]
    pub account_id: cct::Identifier,
}

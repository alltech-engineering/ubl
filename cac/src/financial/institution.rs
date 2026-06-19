#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a financial institution.
///
/// UBL Dictionary Entry Name: `Financial Institution. Details`
///
/// Generated from XSD type `FinancialInstitutionType`.
pub struct FinancialInstitution {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this financial institution. It is recommended that the ISO 9362 Bank
/// Identification Code (BIC) be used as the ID.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this financial institution.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The address of this financial institution.
    #[serde(default, rename = "Address")]
    pub address: Option<crate::Address>,
}

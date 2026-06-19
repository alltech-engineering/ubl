#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a branch or a division of an organization.
///
/// UBL Dictionary Entry Name: `Branch. Details`
///
/// Generated from XSD type `BranchType`.
pub struct Branch {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this branch or division of an organization.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this branch or division of an organization.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The financial institution that this branch belongs to (if applicable).
    #[serde(default, rename = "FinancialInstitution")]
    pub financial_institution: Option<FinancialInstitution>,
/// The address of this branch or division.
    #[serde(default, rename = "Address")]
    pub address: Option<Address>,
}

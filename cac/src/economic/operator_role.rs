#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the tenderer contracting role.
///
/// UBL Dictionary Entry Name: `Economic Operator Role. Details`
///
/// Generated from XSD type `EconomicOperatorRoleType`.
pub struct EconomicOperatorRole {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code specifying the role of the party.
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<cct::Code>,
/// A textual description of the party role.
    #[serde(default, rename = "RoleDescription")]
    pub role_description: Vec<cct::Text>,
}

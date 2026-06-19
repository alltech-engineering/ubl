#[derive(Debug, Deserialize, Serialize)]
/// A class for assigning identifying information for a property
///
/// UBL Dictionary Entry Name: `Property Identification. Details`
///
/// Generated from XSD type `PropertyIdentificationType`.
pub struct PropertyIdentification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An Identifier for the property.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A scope within which the issuer has assigned this identifier.
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<cct::Identifier>,
/// The party that issued this property identifier.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}

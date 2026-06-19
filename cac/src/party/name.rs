#[derive(Debug, Deserialize, Serialize)]
/// A class for defining the name of a party.
///
/// UBL Dictionary Entry Name: `Party Name. Details`
///
/// Generated from XSD type `PartyNameType`.
pub struct PartyName {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The name of the party.
    #[serde(rename = "Name")]
    pub name: cct::Text,
}

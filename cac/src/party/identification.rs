#[derive(Debug, Deserialize, Serialize)]
/// A class to define an identifier for a party.
///
/// UBL Dictionary Entry Name: `Party Identification. Details`
///
/// Generated from XSD type `PartyIdentificationType`.
pub struct PartyIdentification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the party.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
}

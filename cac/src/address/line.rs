#[derive(Debug, Deserialize, Serialize)]
/// A class to define an unstructured address line.
///
/// UBL Dictionary Entry Name: `Address Line. Details`
///
/// Generated from XSD type `AddressLineType`.
pub struct AddressLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An address line expressed as unstructured text (may be repeated only to provide the same content in
/// multiple natural languages).
    #[serde(default, rename = "Line")]
    pub line: Vec<cct::Text>,
}

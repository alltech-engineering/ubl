#[derive(Debug, Deserialize, Serialize)]
/// A class to define a clause (a distinct article or provision) in a contract, treaty, will, or other
/// formal or legal written document requiring compliance.
///
/// UBL Dictionary Entry Name: `Clause. Details`
///
/// Generated from XSD type `ClauseType`.
pub struct Clause {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this clause.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The text of this clause.
    #[serde(default, rename = "Content")]
    pub content: Vec<cct::Text>,
}

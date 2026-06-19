#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a security measure
///
/// UBL Dictionary Entry Name: `Security Measure. Details`
///
/// Generated from XSD type `SecurityMeasureType`.
pub struct SecurityMeasure {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this security measure.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The description of this security measure
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}

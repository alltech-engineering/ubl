#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a fuel property.
///
/// UBL Dictionary Entry Name: `Fuel Property. Details`
///
/// Generated from XSD type `FuelPropertyType`.
pub struct FuelProperty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the type of the fuel property.
    #[serde(rename = "TypeID")]
    pub type_id: cct::Identifier,
/// The value of this fuel property.
    #[serde(rename = "Value")]
    pub value: cct::Text,
}

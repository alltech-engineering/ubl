#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a country.
///
/// UBL Dictionary Entry Name: `Country. Details`
///
/// Generated from XSD type `CountryType`.
pub struct Country {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying this country.
    #[serde(default, rename = "IdentificationCode")]
    pub identification_code: Option<cct::Code>,
/// The name of this country.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
}

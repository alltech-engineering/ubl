#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a reference to a piece of legislation.
///
/// UBL Dictionary Entry Name: `Legislation. Details`
///
/// Generated from XSD type `LegislationType`.
pub struct Legislation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier to refer to the legislation.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The title of the legislation.
    #[serde(default, rename = "Title")]
    pub title: Vec<cct::Text>,
/// The textual description of the legislation.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The jurisdiction level for the legislation.
    #[serde(default, rename = "JurisdictionLevel")]
    pub jurisdiction_level: Vec<cct::Text>,
/// The article of the legislation.
    #[serde(default, rename = "Article")]
    pub article: Vec<cct::Text>,
/// A URI to the legislation.
    #[serde(default, rename = "URI")]
    pub uri: Vec<cct::Identifier>,
/// The language of the legislation.
    #[serde(default, rename = "Language")]
    pub language: Vec<Language>,
/// The geopolitical region in which this legislation applies.
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}

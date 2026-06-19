#[derive(Debug, Deserialize, Serialize)]
/// A character string to identify and uniquely distinguish one instance of an object in an
/// identification scheme from all other objects in the same scheme, together with relevant
/// supplementary information.
///
/// UBL Dictionary Entry Name: `Identifier. Type`
///
/// Generated from XSD type `IdentifierType`.
pub struct Identifier {
    #[serde(default, rename = "@schemeID")]
    pub scheme_id: Option<String>,
/// (Deprecated) The name of the identification scheme.
    #[serde(default, rename = "@schemeName")]
    pub scheme_name: Option<String>,
/// (Deprecated) The identification of the agency that maintains the identification scheme.
    #[serde(default, rename = "@schemeAgencyID")]
    pub scheme_agency_id: Option<String>,
/// (Deprecated) The name of the agency that maintains the identification scheme.
    #[serde(default, rename = "@schemeAgencyName")]
    pub scheme_agency_name: Option<String>,
/// (Deprecated) The version of the identification scheme.
    #[serde(default, rename = "@schemeVersionID")]
    pub scheme_version_id: Option<String>,
/// (Deprecated) The Uniform Resource Identifier that identifies where the identification scheme data is
/// located.
    #[serde(default, rename = "@schemeDataURI")]
    pub scheme_data_uri: Option<String>,
/// (Deprecated) The Uniform Resource Identifier that identifies where the identification scheme is
/// located.
    #[serde(default, rename = "@schemeURI")]
    pub scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}

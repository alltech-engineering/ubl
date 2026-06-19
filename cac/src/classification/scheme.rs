#[derive(Debug, Deserialize, Serialize)]
/// A class to define a classification scheme, such as a taxonomy for classifying goods or services.
///
/// UBL Dictionary Entry Name: `Classification Scheme. Details`
///
/// Generated from XSD type `ClassificationSchemeType`.
pub struct ClassificationScheme {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this classification scheme.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this classification scheme.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date on which this classification scheme was last revised.
    #[serde(default, rename = "LastRevisionDate")]
    pub last_revision_date: Option<udt::DateTime>,
/// The time at which this classification scheme was last revised.
    #[serde(default, rename = "LastRevisionTime")]
    pub last_revision_time: Option<udt::DateTime>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The name of this classification scheme.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// Text describing this classification scheme.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for the agency that maintains this classification scheme.
    #[serde(default, rename = "AgencyID")]
    pub agency_id: Option<cct::Identifier>,
/// The name of the agency that maintains the classification scheme.
    #[serde(default, rename = "AgencyName")]
    pub agency_name: Option<cct::Text>,
/// An identifier for the version of this classification scheme.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The Uniform Resource Identifier (URI) of the documentation for this classification scheme.
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
/// The Uniform Resource Identifier (URI) of this classification scheme.
    #[serde(default, rename = "SchemeURI")]
    pub scheme_uri: Option<cct::Identifier>,
/// An identifier for the language of this classification scheme.
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<cct::Identifier>,
/// A description of a category within this classification scheme.
    #[serde(default, rename = "ClassificationCategory")]
    pub classification_category: Vec<ClassificationCategory>,
}

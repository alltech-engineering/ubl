#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a catalogue.
///
/// UBL Dictionary Entry Name: `Catalogue Reference. Details`
///
/// Generated from XSD type `CatalogueReferenceType`.
pub struct CatalogueReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for a specific catalogue.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for a specific catalogue.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date on which the catalogue was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time at which the catalogue was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The date on which the information in the catalogue was last revised.
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: Option<udt::DateTime>,
/// The time at which the information in the catalogue was last revised.
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: Option<udt::DateTime>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Text describing the catalogue.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for the current version of the catalogue.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// An identifier for the previous version of the catalogue that is superseded by this version.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
}

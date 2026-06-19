#[derive(Debug, Deserialize, Serialize)]
/// A character string (letters, figures, or symbols) that for brevity and/or language independence may
/// be used to represent or replace a definitive value or text of an attribute, together with relevant
/// supplementary information.
///
/// UBL Dictionary Entry Name: `Code. Type`
///
/// Generated from XSD type `CodeType`.
pub struct Code {
    #[serde(default, rename = "@listID")]
    pub list_id: Option<String>,
/// (Deprecated) An agency that maintains one or more lists of codes.
    #[serde(default, rename = "@listAgencyID")]
    pub list_agency_id: Option<String>,
/// (Deprecated) The name of the agency that maintains the list of codes.
    #[serde(default, rename = "@listAgencyName")]
    pub list_agency_name: Option<String>,
/// (Deprecated) The name of a list of codes.
    #[serde(default, rename = "@listName")]
    pub list_name: Option<String>,
/// (Deprecated) The version of the list of codes.
    #[serde(default, rename = "@listVersionID")]
    pub list_version_id: Option<String>,
/// (Deprecated) The textual equivalent of the code content component.
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
/// (Deprecated) The identifier of the language used in the code name.
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
/// (Deprecated) The Uniform Resource Identifier that identifies where the code list is located.
    #[serde(default, rename = "@listURI")]
    pub list_uri: Option<String>,
/// (Deprecated) The Uniform Resource Identifier that identifies where the code list scheme is located.
    #[serde(default, rename = "@listSchemeURI")]
    pub list_scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
///
/// Generated from XSD type `ExtensionAgencyIDType`.
pub struct ExtensionAgencyId {
    #[serde(default, rename = "@schemeID")]
    pub scheme_id: Option<String>,
    #[serde(default, rename = "@schemeName")]
    pub scheme_name: Option<String>,
    #[serde(default, rename = "@schemeAgencyID")]
    pub scheme_agency_id: Option<String>,
    #[serde(default, rename = "@schemeAgencyName")]
    pub scheme_agency_name: Option<String>,
    #[serde(default, rename = "@schemeVersionID")]
    pub scheme_version_id: Option<String>,
    #[serde(default, rename = "@schemeDataURI")]
    pub scheme_data_uri: Option<String>,
    #[serde(default, rename = "@schemeURI")]
    pub scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize)]
///
/// Generated from XSD type `ExtensionAgencyNameType`.
pub struct ExtensionAgencyName {
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type ExtensionAgencyUri = ExtensionAgencyId;
#[derive(Debug, Deserialize, Serialize)]
///
/// Generated from XSD type `ExtensionReasonCodeType`.
pub struct ExtensionReasonCode {
    #[serde(default, rename = "@listID")]
    pub list_id: Option<String>,
    #[serde(default, rename = "@listAgencyID")]
    pub list_agency_id: Option<String>,
    #[serde(default, rename = "@listAgencyName")]
    pub list_agency_name: Option<String>,
    #[serde(default, rename = "@listName")]
    pub list_name: Option<String>,
    #[serde(default, rename = "@listVersionID")]
    pub list_version_id: Option<String>,
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
    #[serde(default, rename = "@listURI")]
    pub list_uri: Option<String>,
    #[serde(default, rename = "@listSchemeURI")]
    pub list_scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type ExtensionVersionId = ExtensionAgencyId;
#[derive(Debug, Deserialize, Serialize)]
/// A single extension for private use.
///
/// Generated from XSD type `UBLExtensionType`.
pub struct UblExtension {
/// An identifier for the Extension assigned by the creator of the extension.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A name for the Extension assigned by the creator of the extension.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// An agency that maintains one or more Extensions.
    #[serde(default, rename = "ExtensionAgencyID")]
    pub extension_agency_id: Option<ExtensionAgencyId>,
/// The name of the agency that maintains the Extension.
    #[serde(default, rename = "ExtensionAgencyName")]
    pub extension_agency_name: Option<ExtensionAgencyName>,
/// The version of the Extension.
    #[serde(default, rename = "ExtensionVersionID")]
    pub extension_version_id: Option<ExtensionAgencyId>,
/// A URI for the Agency that maintains the Extension.
    #[serde(default, rename = "ExtensionAgencyURI")]
    pub extension_agency_uri: Option<ExtensionAgencyId>,
/// A URI for the Extension.
    #[serde(default, rename = "ExtensionURI")]
    pub extension_uri: Option<ExtensionAgencyId>,
/// A code for reason the Extension is being included.
    #[serde(default, rename = "ExtensionReasonCode")]
    pub extension_reason_code: Option<ExtensionReasonCode>,
/// A description of the reason for the Extension.
    #[serde(default, rename = "ExtensionReason")]
    pub extension_reason: Option<ExtensionAgencyName>,
/// The definition of the extension content.
    #[serde(rename = "ExtensionContent")]
    pub extension_content: ExtensionContent,
}
#[derive(Debug, Deserialize, Serialize)]
/// A container for all extensions present in the document.
///
/// Generated from XSD type `UBLExtensionsType`.
pub struct UblExtensions {
/// A single extension for private use.
    #[serde(default, rename = "UBLExtension")]
    pub ubl_extension: Vec<UblExtension>,
}

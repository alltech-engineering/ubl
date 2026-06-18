use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
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
pub struct ExtensionAgencyName {
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type ExtensionAgencyUri = ExtensionAgencyId;
pub type ExtensionAgencyUriType = ExtensionAgencyId;
pub type ExtensionContent = super::ubl_extension_content_data_type_25::ExtensionContentType;
pub type ExtensionReason = ExtensionAgencyName;
#[derive(Debug, Deserialize, Serialize)]
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
pub type ExtensionReasonType = ExtensionAgencyName;
pub type ExtensionUri = ExtensionAgencyId;
pub type ExtensionUriType = ExtensionAgencyId;
pub type ExtensionVersionId = ExtensionAgencyId;
pub type ExtensionVersionIdType = ExtensionAgencyId;
#[derive(Debug, Deserialize, Serialize)]
pub struct UblExtension {
    #[serde(default, rename = "ID")]
    pub id: Option<super::super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::super::cct::TextType>,
    #[serde(default, rename = "ExtensionAgencyID")]
    pub extension_agency_id: Option<ExtensionAgencyId>,
    #[serde(default, rename = "ExtensionAgencyName")]
    pub extension_agency_name: Option<ExtensionAgencyName>,
    #[serde(default, rename = "ExtensionVersionID")]
    pub extension_version_id: Option<ExtensionAgencyId>,
    #[serde(default, rename = "ExtensionAgencyURI")]
    pub extension_agency_uri: Option<ExtensionAgencyId>,
    #[serde(default, rename = "ExtensionURI")]
    pub extension_uri: Option<ExtensionAgencyId>,
    #[serde(default, rename = "ExtensionReasonCode")]
    pub extension_reason_code: Option<ExtensionReasonCode>,
    #[serde(default, rename = "ExtensionReason")]
    pub extension_reason: Option<ExtensionAgencyName>,
    #[serde(rename = "ExtensionContent")]
    pub extension_content: super::ubl_extension_content_data_type_25::ExtensionContentType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UblExtensions {
    #[serde(default, rename = "UBLExtension")]
    pub ubl_extension: Vec<UblExtension>,
}

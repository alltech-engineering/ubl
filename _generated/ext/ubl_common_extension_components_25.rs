use serde::{Deserialize, Serialize};
pub type ExtensionAgencyId = ExtensionAgencyIdType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionAgencyIdType {
    #[serde(default, rename = "@schemeID")]
    pub scheme_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeName")]
    pub scheme_name: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeAgencyID")]
    pub scheme_agency_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeAgencyName")]
    pub scheme_agency_name: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeVersionID")]
    pub scheme_version_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeDataURI")]
    pub scheme_data_uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@schemeURI")]
    pub scheme_uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
pub type ExtensionAgencyName = ExtensionAgencyNameType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionAgencyNameType {
    #[serde(default, rename = "@languageID")]
    pub language_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
pub type ExtensionAgencyUri = ExtensionAgencyIdType;
pub type ExtensionAgencyUriType = ExtensionAgencyIdType;
pub type ExtensionContent = super::ubl_extension_content_data_type_25::ExtensionContentType;
pub type ExtensionReason = ExtensionAgencyNameType;
pub type ExtensionReasonCode = ExtensionReasonCodeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionReasonCodeType {
    #[serde(default, rename = "@listID")]
    pub list_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listAgencyID")]
    pub list_agency_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listAgencyName")]
    pub list_agency_name: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listName")]
    pub list_name: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listVersionID")]
    pub list_version_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@languageID")]
    pub language_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listURI")]
    pub list_uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@listSchemeURI")]
    pub list_scheme_uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
pub type ExtensionReasonType = ExtensionAgencyNameType;
pub type ExtensionUri = ExtensionAgencyIdType;
pub type ExtensionUriType = ExtensionAgencyIdType;
pub type ExtensionVersionId = ExtensionAgencyIdType;
pub type ExtensionVersionIdType = ExtensionAgencyIdType;
pub type UblExtension = UblExtensionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UblExtensionType {
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::super::cct::TextType>,
    #[serde(default, rename = "ExtensionAgencyID")]
    pub extension_agency_id: ::core::option::Option<ExtensionAgencyIdType>,
    #[serde(default, rename = "ExtensionAgencyName")]
    pub extension_agency_name: ::core::option::Option<ExtensionAgencyNameType>,
    #[serde(default, rename = "ExtensionVersionID")]
    pub extension_version_id: ::core::option::Option<ExtensionAgencyIdType>,
    #[serde(default, rename = "ExtensionAgencyURI")]
    pub extension_agency_uri: ::core::option::Option<ExtensionAgencyIdType>,
    #[serde(default, rename = "ExtensionURI")]
    pub extension_uri: ::core::option::Option<ExtensionAgencyIdType>,
    #[serde(default, rename = "ExtensionReasonCode")]
    pub extension_reason_code: ::core::option::Option<ExtensionReasonCodeType>,
    #[serde(default, rename = "ExtensionReason")]
    pub extension_reason: ::core::option::Option<ExtensionAgencyNameType>,
    #[serde(rename = "ExtensionContent")]
    pub extension_content: super::ubl_extension_content_data_type_25::ExtensionContentType,
}
pub type UblExtensions = UblExtensionsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UblExtensionsType {
    #[serde(default, rename = "UBLExtension")]
    pub ubl_extension: ::std::vec::Vec<UblExtensionType>,
}

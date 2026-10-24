use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AmountType {
    #[serde(default, rename = "@currencyID")]
    pub currency_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@currencyCodeListVersionID")]
    pub currency_code_list_version_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::core::primitive::f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryObjectType {
    #[serde(default, rename = "@format")]
    pub format: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@mimeCode")]
    pub mime_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@encodingCode")]
    pub encoding_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@characterSetCode")]
    pub character_set_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@uri")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@filename")]
    pub filename: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CodeType {
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
#[derive(Debug, Deserialize, Serialize)]
pub struct DateTimeType {
    #[serde(default, rename = "@format")]
    pub format: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifierType {
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
#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorType {
    #[serde(default, rename = "@format")]
    pub format: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MeasureType {
    #[serde(default, rename = "@unitCode")]
    pub unit_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@unitCodeListVersionID")]
    pub unit_code_list_version_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::core::primitive::f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NumericType {
    #[serde(default, rename = "@format")]
    pub format: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::core::primitive::f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct QuantityType {
    #[serde(default, rename = "@unitCode")]
    pub unit_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@unitCodeListID")]
    pub unit_code_list_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@unitCodeListAgencyID")]
    pub unit_code_list_agency_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@unitCodeListAgencyName")]
    pub unit_code_list_agency_name: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::core::primitive::f64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TextType {
    #[serde(default, rename = "@languageID")]
    pub language_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}

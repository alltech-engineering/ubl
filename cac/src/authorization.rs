#[derive(Debug, Deserialize, Serialize)]
/// A class to define an authorization that as been issued
///
/// UBL Dictionary Entry Name: `Authorization. Details`
///
/// Generated from XSD type `AuthorizationType`.
pub struct Authorization {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code defining the business purpose or scope of this authorization
    #[serde(default, rename = "PurposeCode")]
    pub purpose_code: Option<cct::Code>,
/// The purpose or scope of this authorization expressed as a text
    #[serde(default, rename = "Purpose")]
    pub purpose: Vec<cct::Text>,
/// The period during which this authorization is valid
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
/// One or more certificates related to this authorization
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}

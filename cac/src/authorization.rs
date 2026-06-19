#[derive(Debug, Deserialize, Serialize)]
pub struct Authorization {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PurposeCode")]
    pub purpose_code: Option<cct::Code>,
    #[serde(default, rename = "Purpose")]
    pub purpose: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Authorization {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PurposeCode")]
    pub purpose_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Purpose")]
    pub purpose: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}

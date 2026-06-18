#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalAgreementTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "AdoptionPeriod")]
    pub adoption_period: Option<Period>,
    #[serde(default, rename = "ServiceLevelAgreement")]
    pub service_level_agreement: Vec<ServiceLevelAgreement>,
}

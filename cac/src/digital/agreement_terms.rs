#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalAgreementTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
    #[serde(default, rename = "AdoptionPeriod")]
    pub adoption_period: Option<crate::Period>,
    #[serde(default, rename = "ServiceLevelAgreement")]
    pub service_level_agreement: Vec<crate::ServiceLevelAgreement>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the terms and conditions of a digital agreement.
///
/// UBL Dictionary Entry Name: `Digital Agreement Terms. Details`
///
/// Generated from XSD type `DigitalAgreementTermsType`.
pub struct DigitalAgreementTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// Text describing the terms and conditions of a digital agreement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period of time for which this digital agreement is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
/// The period during which a digital agreement must be adopted.
    #[serde(default, rename = "AdoptionPeriod")]
    pub adoption_period: Option<crate::Period>,
/// The service level agreement which regulates the quality, availability and responsibilities of
/// digital services.
    #[serde(default, rename = "ServiceLevelAgreement")]
    pub service_level_agreement: Vec<crate::ServiceLevelAgreement>,
}

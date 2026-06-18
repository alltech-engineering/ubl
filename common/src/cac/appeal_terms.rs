#[derive(Debug, Deserialize, Serialize)]
pub struct AppealTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PresentationPeriod")]
    pub presentation_period: Option<Period>,
    #[serde(default, rename = "AppealInformationParty")]
    pub appeal_information_party: Option<Party>,
    #[serde(default, rename = "AppealReceiverParty")]
    pub appeal_receiver_party: Option<Party>,
    #[serde(default, rename = "MediationParty")]
    pub mediation_party: Option<Party>,
}

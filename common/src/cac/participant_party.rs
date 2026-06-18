#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "InitiatingPartyIndicator")]
    pub initiating_party_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrivatePartyIndicator")]
    pub private_party_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PublicPartyIndicator")]
    pub public_party_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ServiceProviderPartyIndicator")]
    pub service_provider_party_indicator: Option<super::udt::IndicatorType>,
    #[serde(rename = "Party")]
    pub party: Party,
    #[serde(default, rename = "LegalContact")]
    pub legal_contact: Option<Contact>,
    #[serde(default, rename = "TechnicalContact")]
    pub technical_contact: Option<Contact>,
    #[serde(default, rename = "SupportContact")]
    pub support_contact: Option<Contact>,
    #[serde(default, rename = "CommercialContact")]
    pub commercial_contact: Option<Contact>,
}

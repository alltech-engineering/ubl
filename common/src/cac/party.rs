#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MarkCareIndicator")]
    pub mark_care_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MarkAttentionIndicator")]
    pub mark_attention_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "WebsiteURI")]
    pub website_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LogoReferenceID")]
    pub logo_reference_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "EndpointID")]
    pub endpoint_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IndustryClassificationCode")]
    pub industry_classification_code: Vec<super::cct::CodeType>,
    #[serde(default, rename = "PartyIdentification")]
    pub party_identification: Vec<PartyIdentification>,
    #[serde(default, rename = "AdditionalPartyIdentification")]
    pub additional_party_identification: Vec<PartyIdentification>,
    #[serde(default, rename = "PartyName")]
    pub party_name: Vec<PartyName>,
    #[serde(default, rename = "TradePartyName")]
    pub trade_party_name: Vec<PartyName>,
    #[serde(default, rename = "Language")]
    pub language: Option<Language>,
    #[serde(default, rename = "PostalAddress")]
    pub postal_address: Option<Address>,
    #[serde(default, rename = "PhysicalLocation")]
    pub physical_location: Option<Location>,
    #[serde(default, rename = "PartyTaxScheme")]
    pub party_tax_scheme: Vec<PartyTaxScheme>,
    #[serde(default, rename = "PartyLegalEntity")]
    pub party_legal_entity: Vec<PartyLegalEntity>,
    #[serde(default, rename = "Contact")]
    pub contact: Option<Contact>,
    #[serde(default, rename = "Person")]
    pub person: Vec<Person>,
    #[serde(default, rename = "AgentParty")]
    pub agent_party: Option<Box<Party>>,
    #[serde(default, rename = "ServiceProviderParty")]
    pub service_provider_party: Vec<ServiceProviderParty>,
    #[serde(default, rename = "PowerOfAttorney")]
    pub power_of_attorney: Vec<PowerOfAttorney>,
    #[serde(default, rename = "PartyAuthorization")]
    pub party_authorization: Vec<Authorization>,
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "AdditionalWebSite")]
    pub additional_web_site: Vec<WebSite>,
    #[serde(default, rename = "SocialMediaProfile")]
    pub social_media_profile: Vec<SocialMediaProfile>,
    #[serde(default, rename = "ElectronicAddress")]
    pub electronic_address: Vec<ElectronicAddress>,
}

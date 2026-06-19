use serde::{Deserialize, Serialize};


include!("legal_entity.rs");
include!("identification.rs");
include!("name.rs");
include!("tax_scheme.rs");
include!("group.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Party {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "MarkCareIndicator")]
    pub mark_care_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MarkAttentionIndicator")]
    pub mark_attention_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "WebsiteURI")]
    pub website_uri: Option<cct::Identifier>,
    #[serde(default, rename = "LogoReferenceID")]
    pub logo_reference_id: Option<cct::Identifier>,
    #[serde(default, rename = "EndpointID")]
    pub endpoint_id: Option<cct::Identifier>,
    #[serde(default, rename = "IndustryClassificationCode")]
    pub industry_classification_code: Vec<cct::Code>,
    #[serde(default, rename = "PartyIdentification")]
    pub party_identification: Vec<PartyIdentification>,
    #[serde(default, rename = "AdditionalPartyIdentification")]
    pub additional_party_identification: Vec<PartyIdentification>,
    #[serde(default, rename = "PartyName")]
    pub party_name: Vec<PartyName>,
    #[serde(default, rename = "TradePartyName")]
    pub trade_party_name: Vec<PartyName>,
    #[serde(default, rename = "Language")]
    pub language: Option<crate::Language>,
    #[serde(default, rename = "PostalAddress")]
    pub postal_address: Option<crate::Address>,
    #[serde(default, rename = "PhysicalLocation")]
    pub physical_location: Option<crate::Location>,
    #[serde(default, rename = "PartyTaxScheme")]
    pub party_tax_scheme: Vec<PartyTaxScheme>,
    #[serde(default, rename = "PartyLegalEntity")]
    pub party_legal_entity: Vec<PartyLegalEntity>,
    #[serde(default, rename = "Contact")]
    pub contact: Option<crate::Contact>,
    #[serde(default, rename = "Person")]
    pub person: Vec<crate::Person>,
    #[serde(default, rename = "AgentParty")]
    pub agent_party: Option<Box<Party>>,
    #[serde(default, rename = "ServiceProviderParty")]
    pub service_provider_party: Vec<crate::ServiceProviderParty>,
    #[serde(default, rename = "PowerOfAttorney")]
    pub power_of_attorney: Vec<crate::PowerOfAttorney>,
    #[serde(default, rename = "PartyAuthorization")]
    pub party_authorization: Vec<crate::Authorization>,
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: Option<crate::FinancialAccount>,
    #[serde(default, rename = "AdditionalWebSite")]
    pub additional_web_site: Vec<crate::WebSite>,
    #[serde(default, rename = "SocialMediaProfile")]
    pub social_media_profile: Vec<crate::SocialMediaProfile>,
    #[serde(default, rename = "ElectronicAddress")]
    pub electronic_address: Vec<crate::ElectronicAddress>,
}

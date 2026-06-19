use serde::{Deserialize, Serialize};


include!("legal_entity.rs");
include!("identification.rs");
include!("name.rs");
include!("tax_scheme.rs");
include!("group.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an organization, sub-organization, or individual fulfilling a role in a business
/// process.
///
/// UBL Dictionary Entry Name: `Party. Details`
///
/// Generated from XSD type `PartyType`.
pub struct Party {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// (Deprecated) An indicator that this party is "care of" (c/o) (true) or not (false).
    #[serde(default, rename = "MarkCareIndicator")]
    pub mark_care_indicator: Option<udt::Indicator>,
/// (Deprecated) An indicator that this party is "for the attention of" (FAO) (true) or not (false).
    #[serde(default, rename = "MarkAttentionIndicator")]
    pub mark_attention_indicator: Option<udt::Indicator>,
/// The Uniform Resource Identifier (URI) that identifies this party's web site; i.e., the web site's
/// Uniform Resource Locator (URL).
    #[serde(default, rename = "WebsiteURI")]
    pub website_uri: Option<cct::Identifier>,
/// An identifier for this party's logo.
    #[serde(default, rename = "LogoReferenceID")]
    pub logo_reference_id: Option<cct::Identifier>,
/// (Deprecated) An identifier for the end point of the routing service (e.g., EAN Location Number,
/// GLN).
    #[serde(default, rename = "EndpointID")]
    pub endpoint_id: Option<cct::Identifier>,
/// This party's Industry Classification Code.
    #[serde(default, rename = "IndustryClassificationCode")]
    pub industry_classification_code: Vec<cct::Code>,
/// (Endorsed cardinality: 0..1) A preferred identifier for this Party.
    #[serde(default, rename = "PartyIdentification")]
    pub party_identification: Vec<PartyIdentification>,
/// One or more additional identifiers for this Party.
    #[serde(default, rename = "AdditionalPartyIdentification")]
    pub additional_party_identification: Vec<PartyIdentification>,
/// (Endorsed cardinality: 0..1) A name for this party.
    #[serde(default, rename = "PartyName")]
    pub party_name: Vec<PartyName>,
/// A trade name for this Party.
    #[serde(default, rename = "TradePartyName")]
    pub trade_party_name: Vec<PartyName>,
/// The language associated with this party.
    #[serde(default, rename = "Language")]
    pub language: Option<crate::Language>,
/// The party's postal address.
    #[serde(default, rename = "PostalAddress")]
    pub postal_address: Option<crate::Address>,
/// The physical location of this party.
    #[serde(default, rename = "PhysicalLocation")]
    pub physical_location: Option<crate::Location>,
/// A tax scheme applying to this party.
    #[serde(default, rename = "PartyTaxScheme")]
    pub party_tax_scheme: Vec<PartyTaxScheme>,
/// A description of this party as a legal entity.
    #[serde(default, rename = "PartyLegalEntity")]
    pub party_legal_entity: Vec<PartyLegalEntity>,
/// The primary contact for this party.
    #[serde(default, rename = "Contact")]
    pub contact: Option<crate::Contact>,
/// A person associated with this party.
    #[serde(default, rename = "Person")]
    pub person: Vec<crate::Person>,
/// The Party who acts as an Agent for this Party.
    #[serde(default, rename = "AgentParty")]
    pub agent_party: Option<Box<Party>>,
/// A party providing a service to this party.
    #[serde(default, rename = "ServiceProviderParty")]
    pub service_provider_party: Vec<crate::ServiceProviderParty>,
/// A power of attorney associated with this party.
    #[serde(default, rename = "PowerOfAttorney")]
    pub power_of_attorney: Vec<crate::PowerOfAttorney>,
/// An authorization issued to this party
    #[serde(default, rename = "PartyAuthorization")]
    pub party_authorization: Vec<crate::Authorization>,
/// The financial account associated with this party.
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: Option<crate::FinancialAccount>,
/// An additional web site associated with this party (e.g. a satellite web site).
    #[serde(default, rename = "AdditionalWebSite")]
    pub additional_web_site: Vec<crate::WebSite>,
/// A social media profile associated with this party.
    #[serde(default, rename = "SocialMediaProfile")]
    pub social_media_profile: Vec<crate::SocialMediaProfile>,
/// An Electronic Address where this Party is registered.
    #[serde(default, rename = "ElectronicAddress")]
    pub electronic_address: Vec<crate::ElectronicAddress>,
}

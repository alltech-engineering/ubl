#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a participant party.
///
/// UBL Dictionary Entry Name: `Participant Party. Details`
///
/// Generated from XSD type `ParticipantPartyType`.
pub struct ParticipantParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An indicator that this party is playing the role of the initiator within a transaction (true) or not
/// (false).
    #[serde(default, rename = "InitiatingPartyIndicator")]
    pub initiating_party_indicator: Option<udt::Indicator>,
/// An indicator that this party is a private entity (true) or not (false).
    #[serde(default, rename = "PrivatePartyIndicator")]
    pub private_party_indicator: Option<udt::Indicator>,
/// An indicator that this party is a public (governmental) entity (true) or not (false).
    #[serde(default, rename = "PublicPartyIndicator")]
    pub public_party_indicator: Option<udt::Indicator>,
/// An indicator that this party is a service provider (true) or not (false).
    #[serde(default, rename = "ServiceProviderPartyIndicator")]
    pub service_provider_party_indicator: Option<udt::Indicator>,
/// The Party who participates.
    #[serde(rename = "Party")]
    pub party: Party,
/// A legal contact associated to this participant for sending legal notices.
    #[serde(default, rename = "LegalContact")]
    pub legal_contact: Option<Contact>,
/// A technical contact associated to this participant.
    #[serde(default, rename = "TechnicalContact")]
    pub technical_contact: Option<Contact>,
/// A support contact associated to this participant.
    #[serde(default, rename = "SupportContact")]
    pub support_contact: Option<Contact>,
/// A commercial contact associated to this participant.
    #[serde(default, rename = "CommercialContact")]
    pub commercial_contact: Option<Contact>,
}

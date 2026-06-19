use serde::{Deserialize, Serialize};

include!("representation.rs");
include!("party_kind.rs");
include!("system.rs");
include!("activity.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an individual, a group, or a body having a procurement role in a tendering
/// process.
///
/// UBL Dictionary Entry Name: `Contracting Party. Details`
///
/// Generated from XSD type `ContractingPartyType`.
pub struct ContractingParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The buyer profile is typically located on a web site where the contracting party publishes its
/// procurement opportunities
    #[serde(default, rename = "BuyerProfileURI")]
    pub buyer_profile_uri: Option<cct::Identifier>,
/// The type of contracting party that is independent of its role.
    #[serde(default, rename = "ContractingPartyType")]
    pub contracting_party_type: Vec<ContractingPartyKind>,
/// The nature of the type of business of the organization
    #[serde(default, rename = "ContractingActivity")]
    pub contracting_activity: Vec<ContractingActivity>,
/// The type of represention empowering the party to act on behalf of a third party
    #[serde(default, rename = "ContractingRepresentationType")]
    pub contracting_representation_type: Option<ContractingRepresentation>,
/// The Party who is reponsible for the Contract.
    #[serde(rename = "Party")]
    pub party: crate::Party,
}

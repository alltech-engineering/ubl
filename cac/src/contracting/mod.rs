use serde::{Deserialize, Serialize};

include!("representation.rs");
include!("party_kind.rs");
include!("system.rs");
include!("activity.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "BuyerProfileURI")]
    pub buyer_profile_uri: Option<cct::Identifier>,
    #[serde(default, rename = "ContractingPartyType")]
    pub contracting_party_type: Vec<ContractingPartyKind>,
    #[serde(default, rename = "ContractingActivity")]
    pub contracting_activity: Vec<ContractingActivity>,
    #[serde(default, rename = "ContractingRepresentationType")]
    pub contracting_representation_type: Option<ContractingRepresentation>,
    #[serde(rename = "Party")]
    pub party: crate::Party,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "BuyerProfileURI")]
    pub buyer_profile_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContractingPartyType")]
    pub contracting_party_type: Vec<ContractingPartyTypeType>,
    #[serde(default, rename = "ContractingActivity")]
    pub contracting_activity: Vec<ContractingActivity>,
    #[serde(default, rename = "ContractingRepresentationType")]
    pub contracting_representation_type: Option<ContractingRepresentationType>,
    #[serde(rename = "Party")]
    pub party: Party,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingPartyKind {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "PartyTypeCode")]
    pub party_type_code: Option<cct::Code>,
    #[serde(default, rename = "PartyType")]
    pub party_type: Vec<cct::Text>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingPartyTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PartyTypeCode")]
    pub party_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PartyType")]
    pub party_type: Vec<super::cct::TextType>,
}

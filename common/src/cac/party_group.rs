#[derive(Debug, Deserialize, Serialize)]
pub struct PartyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "GroupTypeCode")]
    pub group_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "GroupType")]
    pub group_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "Party")]
    pub party: Vec<Party>,
}

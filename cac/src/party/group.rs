#[derive(Debug, Deserialize, Serialize)]
pub struct PartyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "GroupTypeCode")]
    pub group_type_code: Option<cct::Code>,
    #[serde(default, rename = "GroupType")]
    pub group_type: Vec<cct::Text>,
    #[serde(default, rename = "Party")]
    pub party: Vec<Party>,
}

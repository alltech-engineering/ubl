#[derive(Debug, Deserialize, Serialize)]
pub struct WinningParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Rank")]
    pub rank: Option<super::cct::TextType>,
    #[serde(rename = "Party")]
    pub party: Party,
}

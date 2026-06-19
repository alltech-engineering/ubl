#[derive(Debug, Deserialize, Serialize)]
pub struct WinningParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Rank")]
    pub rank: Option<cct::Text>,
    #[serde(rename = "Party")]
    pub party: Party,
}

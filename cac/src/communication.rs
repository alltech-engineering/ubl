#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ChannelCode")]
    pub channel_code: Option<cct::Code>,
    #[serde(default, rename = "Channel")]
    pub channel: Option<cct::Text>,
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
}

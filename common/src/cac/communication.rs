#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ChannelCode")]
    pub channel_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Channel")]
    pub channel: Option<super::cct::TextType>,
    #[serde(default, rename = "Value")]
    pub value: Option<super::cct::TextType>,
}

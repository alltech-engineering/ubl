#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDelivery {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ProtocolID")]
    pub protocol_id: Option<cct::Identifier>,
    #[serde(default, rename = "EnvelopeTypeCode")]
    pub envelope_type_code: Option<cct::Code>,
    #[serde(default, rename = "EndpointURI")]
    pub endpoint_uri: Option<cct::Identifier>,
}

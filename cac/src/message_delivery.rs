#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDelivery {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ProtocolID")]
    pub protocol_id: Option<cct::Identifier>,
    #[serde(default, rename = "EnvelopeTypeCode")]
    pub envelope_type_code: Option<cct::Code>,
    #[serde(default, rename = "EndpointURI")]
    pub endpoint_uri: Option<cct::Identifier>,
}

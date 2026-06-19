#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryChannel {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<cct::Identifier>,
    #[serde(default, rename = "ParticipantID")]
    pub participant_id: Option<cct::Identifier>,
    #[serde(default, rename = "TestIndicator")]
    pub test_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DigitalCertificate")]
    pub digital_certificate: Option<crate::Certificate>,
    #[serde(default, rename = "DigitalMessageDelivery")]
    pub digital_message_delivery: Option<crate::MessageDelivery>,
}

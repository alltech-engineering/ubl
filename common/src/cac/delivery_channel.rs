#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryChannel {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ParticipantID")]
    pub participant_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TestIndicator")]
    pub test_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DigitalCertificate")]
    pub digital_certificate: Option<Certificate>,
    #[serde(default, rename = "DigitalMessageDelivery")]
    pub digital_message_delivery: Option<MessageDelivery>,
}

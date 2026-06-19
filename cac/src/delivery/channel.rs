#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a delivery channel.
///
/// UBL Dictionary Entry Name: `Delivery Channel. Details`
///
/// Generated from XSD type `DeliveryChannelType`.
pub struct DeliveryChannel {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the network where messages are delivered (e.g. a business network).
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<cct::Identifier>,
/// An identifier for a registered participant in the network (e.g. according a precise scheme such as
/// IT:VAT, DK:CVR, GLN).
    #[serde(default, rename = "ParticipantID")]
    pub participant_id: Option<cct::Identifier>,
/// An indicator that the channel is a test channel (true).
    #[serde(default, rename = "TestIndicator")]
    pub test_indicator: Option<udt::Indicator>,
/// A digital certificate associated with this delivery channel.
    #[serde(default, rename = "DigitalCertificate")]
    pub digital_certificate: Option<crate::Certificate>,
/// A digital message delivery associated with this delivery channel (aka routing information).
    #[serde(default, rename = "DigitalMessageDelivery")]
    pub digital_message_delivery: Option<crate::MessageDelivery>,
}

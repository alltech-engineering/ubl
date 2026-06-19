#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a means of communication.
///
/// UBL Dictionary Entry Name: `Communication. Details`
///
/// Generated from XSD type `CommunicationType`.
pub struct Communication {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The method of communication, expressed as a code.
    #[serde(default, rename = "ChannelCode")]
    pub channel_code: Option<cct::Code>,
/// The method of communication, expressed as text.
    #[serde(default, rename = "Channel")]
    pub channel: Option<cct::Text>,
/// An identifying value (phone number, email address, etc.) for this channel of communication
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe how a message is delivered (routed).
///
/// UBL Dictionary Entry Name: `Message Delivery. Details`
///
/// Generated from XSD type `MessageDeliveryType`.
pub struct MessageDelivery {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the protocol to be used within this message delivery.
    #[serde(default, rename = "ProtocolID")]
    pub protocol_id: Option<cct::Identifier>,
/// A code signifying the type of envelope to be used within this message delivery (e.g. OASIS BDX
/// Business Document Envelope).
    #[serde(default, rename = "EnvelopeTypeCode")]
    pub envelope_type_code: Option<cct::Code>,
/// The Uniform Resource Identifier (URI) of the access point (e.g. an HTTP URL including the port).
    #[serde(default, rename = "EndpointURI")]
    pub endpoint_uri: Option<cct::Identifier>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a specific digital trade service supported by an organization for either sending
/// or receiving business documents on different formats (e.g. UBL, ISO20022, EDIFACT, ...).
///
/// UBL Dictionary Entry Name: `Digital Service. Details`
///
/// Generated from XSD type `DigitalServiceType`.
pub struct DigitalService {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the digital service (aka transaction ID).
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Identifies a user-defined customization of this digital service (e.g. a PEPPOL customization).
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// The digital document metadata associated with this digital service.
    #[serde(default, rename = "DigitalDocumentMetadata")]
    pub digital_document_metadata: Vec<crate::DocumentMetadata>,
/// The digital delivery channel associated with this digital service.
    #[serde(default, rename = "DigitalDeliveryChannel")]
    pub digital_delivery_channel: Vec<crate::DeliveryChannel>,
/// A reference to a certification document associated with this digital service.
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: Vec<crate::DocumentReference>,
}

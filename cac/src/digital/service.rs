#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalService {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "DigitalDocumentMetadata")]
    pub digital_document_metadata: Vec<crate::DocumentMetadata>,
    #[serde(default, rename = "DigitalDeliveryChannel")]
    pub digital_delivery_channel: Vec<crate::DeliveryChannel>,
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: Vec<crate::DocumentReference>,
}

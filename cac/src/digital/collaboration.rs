#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a digital trade collaboration.
///
/// UBL Dictionary Entry Name: `Digital Collaboration. Details`
///
/// Generated from XSD type `DigitalCollaborationType`.
pub struct DigitalCollaboration {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the digital collaboration.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The sending digital service associated with this digital collaboration.
    #[serde(default, rename = "SendingDigitalService")]
    pub sending_digital_service: Option<DigitalService>,
/// The receiving digital service associated with this digital collaboration.
    #[serde(default, rename = "ReceivingDigitalService")]
    pub receiving_digital_service: Option<DigitalService>,
}

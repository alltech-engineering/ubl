#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalCollaboration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SendingDigitalService")]
    pub sending_digital_service: Option<DigitalService>,
    #[serde(default, rename = "ReceivingDigitalService")]
    pub receiving_digital_service: Option<DigitalService>,
}

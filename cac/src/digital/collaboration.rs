#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalCollaboration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "SendingDigitalService")]
    pub sending_digital_service: Option<DigitalService>,
    #[serde(default, rename = "ReceivingDigitalService")]
    pub receiving_digital_service: Option<DigitalService>,
}

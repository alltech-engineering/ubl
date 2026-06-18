#[derive(Debug, Deserialize, Serialize)]
pub struct CashRegister {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SerialNumberID")]
    pub serial_number_id: Option<super::cct::IdentifierType>,
}

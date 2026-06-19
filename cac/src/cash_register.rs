#[derive(Debug, Deserialize, Serialize)]
pub struct CashRegister {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "SerialNumberID")]
    pub serial_number_id: Option<cct::Identifier>,
}

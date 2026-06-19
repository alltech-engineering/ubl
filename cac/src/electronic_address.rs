#[derive(Debug, Deserialize, Serialize)]
pub struct ElectronicAddress {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ExchangeNetworkID")]
    pub exchange_network_id: Option<cct::Identifier>,
    #[serde(rename = "ElectronicAddressID")]
    pub electronic_address_id: cct::Identifier,
}

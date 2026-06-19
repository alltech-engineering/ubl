#[derive(Debug, Deserialize, Serialize)]
pub struct ElectronicAddress {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ExchangeNetworkID")]
    pub exchange_network_id: Option<cct::Identifier>,
    #[serde(rename = "ElectronicAddressID")]
    pub electronic_address_id: cct::Identifier,
}

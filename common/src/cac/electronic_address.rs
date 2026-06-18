#[derive(Debug, Deserialize, Serialize)]
pub struct ElectronicAddress {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ExchangeNetworkID")]
    pub exchange_network_id: Option<super::cct::IdentifierType>,
    #[serde(rename = "ElectronicAddressID")]
    pub electronic_address_id: super::cct::IdentifierType,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an Electronic Address where a Party is registered on a given exchange network.
///
/// UBL Dictionary Entry Name: `Electronic Address. Details`
///
/// Generated from XSD type `ElectronicAddressType`.
pub struct ElectronicAddress {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the exchange network where the Party is registered.
    #[serde(default, rename = "ExchangeNetworkID")]
    pub exchange_network_id: Option<cct::Identifier>,
/// An identifier for the Electronic Address of the Party on the given exchange network.
    #[serde(rename = "ElectronicAddressID")]
    pub electronic_address_id: cct::Identifier,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class describing the supply of a telecommunication service, e.g., providing telephone calls.
///
/// UBL Dictionary Entry Name: `Telecommunications Supply. Details`
///
/// Generated from XSD type `TelecommunicationsSupplyType`.
pub struct TelecommunicationsSupply {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The type of telecommunications supply, expressed as text.
    #[serde(default, rename = "TelecommunicationsSupplyType")]
    pub telecommunications_supply_type: Option<cct::Text>,
/// The type of telecommunications supply, expressed as a code.
    #[serde(default, rename = "TelecommunicationsSupplyTypeCode")]
    pub telecommunications_supply_type_code: Option<cct::Code>,
/// A code signifying the level of confidentiality of this information for this telecommunication
/// supply.
    #[serde(rename = "PrivacyCode")]
    pub privacy_code: cct::Code,
/// Text describing the telecommunications supply.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The total amount associated with this telecommunications supply.
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
/// Outlines the provided telecommunication supply
    #[serde(default, rename = "TelecommunicationsSupplyLine")]
    pub telecommunications_supply_line: Vec<TelecommunicationsSupplyLine>,
}

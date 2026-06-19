#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a device (a transport equipment seal) for securing the doors of a shipping
/// container.
///
/// UBL Dictionary Entry Name: `Transport Equipment Seal. Details`
///
/// Generated from XSD type `TransportEquipmentSealType`.
pub struct TransportEquipmentSeal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this transport equipment seal.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A code signifying the type of party that issues and is responsible for this transport equipment
/// seal.
    #[serde(default, rename = "SealIssuerTypeCode")]
    pub seal_issuer_type_code: Option<cct::Code>,
/// The condition of this transport equipment seal.
    #[serde(default, rename = "Condition")]
    pub condition: Option<cct::Text>,
/// A code signifying the condition of this transport equipment seal.
    #[serde(default, rename = "SealStatusCode")]
    pub seal_status_code: Option<cct::Code>,
/// The role of the sealing party.
    #[serde(default, rename = "SealingPartyType")]
    pub sealing_party_type: Option<cct::Text>,
}

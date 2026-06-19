#[derive(Debug, Deserialize, Serialize)]
/// A class to define the cash register used in a commercial transaction.
///
/// UBL Dictionary Entry Name: `Cash Register. Details`
///
/// Generated from XSD type `CashRegisterType`.
pub struct CashRegister {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The identifier of this cash register.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The serial number of this cash register.
    #[serde(default, rename = "SerialNumberID")]
    pub serial_number_id: Option<cct::Identifier>,
}

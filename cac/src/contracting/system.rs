#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the contracting system. If the procedure is individual (nonrepetitive), this
/// class ought not be used.
///
/// UBL Dictionary Entry Name: `Contracting System. Details`
///
/// Generated from XSD type `ContractingSystemType`.
pub struct ContractingSystem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the contracting system.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of contracting system (e.g., framework agreement, dynamic purchasing
/// system).
    #[serde(default, rename = "ContractingSystemTypeCode")]
    pub contracting_system_type_code: Option<cct::Code>,
/// The description of the contracting system
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}

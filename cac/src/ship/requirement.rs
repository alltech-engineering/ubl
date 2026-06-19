#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a requirement for a ship
///
/// UBL Dictionary Entry Name: `Ship Requirement. Details`
///
/// Generated from XSD type `ShipRequirementType`.
pub struct ShipRequirement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this ship requirement.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The description of this ship requirement
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}

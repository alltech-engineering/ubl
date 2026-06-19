#[derive(Debug, Deserialize, Serialize)]
/// A class for assigning identifying information to an item.
///
/// UBL Dictionary Entry Name: `Item Identification. Details`
///
/// Generated from XSD type `ItemIdentificationType`.
pub struct ItemIdentification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the item.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An extended identifier for the item that identifies the item with specific properties, e.g., Item
/// 123 = Chair / Item 123 Ext 45 = brown chair. Two chairs can have the same item number, but one is
/// brown. The other is white.
    #[serde(default, rename = "ExtendedID")]
    pub extended_id: Option<cct::Identifier>,
/// An identifier for a system of barcodes.
    #[serde(default, rename = "BarcodeSymbologyID")]
    pub barcode_symbology_id: Option<cct::Identifier>,
/// A scope within which the issuer has assigned this identifier.
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<cct::Identifier>,
/// A physical attribute of the item.
    #[serde(default, rename = "PhysicalAttribute")]
    pub physical_attribute: Vec<crate::PhysicalAttribute>,
/// A measurable dimension (length, mass, weight, or volume) of the item.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
/// The Party who issues this Item Identification.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ExtendedID")]
    pub extended_id: Option<cct::Identifier>,
    #[serde(default, rename = "BarcodeSymbologyID")]
    pub barcode_symbology_id: Option<cct::Identifier>,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<cct::Identifier>,
    #[serde(default, rename = "PhysicalAttribute")]
    pub physical_attribute: Vec<crate::PhysicalAttribute>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<crate::Party>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ExtendedID")]
    pub extended_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BarcodeSymbologyID")]
    pub barcode_symbology_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PhysicalAttribute")]
    pub physical_attribute: Vec<PhysicalAttribute>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a package.
///
/// UBL Dictionary Entry Name: `Package. Details`
///
/// Generated from XSD type `PackageType`.
pub struct Package {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this package.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The quantity of items contained in this package.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// An indicator that the packaging material is returnable (true) or not (false).
    #[serde(default, rename = "ReturnableMaterialIndicator")]
    pub returnable_material_indicator: Option<udt::Indicator>,
/// A code signifying a level of packaging.
    #[serde(default, rename = "PackageLevelCode")]
    pub package_level_code: Option<cct::Code>,
/// A code signifying a type of packaging.
    #[serde(default, rename = "PackagingTypeCode")]
    pub packaging_type_code: Option<cct::Code>,
/// The type of packaging, described as a text.
    #[serde(default, rename = "PackagingType")]
    pub packaging_type: Vec<cct::Text>,
/// Text describing the packaging material.
    #[serde(default, rename = "PackingMaterial")]
    pub packing_material: Vec<cct::Text>,
/// An identifier for use in tracing this package, such as the EPC number used in RFID.
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
/// A package contained within this package.
    #[serde(default, rename = "ContainedPackage")]
    pub contained_package: Vec<Package>,
/// The piece of transport equipment containing this package.
    #[serde(default, rename = "ContainingTransportEquipment")]
    pub containing_transport_equipment: Option<TransportEquipment>,
/// A goods item included in this package.
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<GoodsItem>,
/// A measurable dimension (length, mass, weight, or volume) of this package.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
/// A delivery unit within this package.
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: Vec<DeliveryUnit>,
/// The delivery of this package.
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Delivery>,
/// The pickup of this package.
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<Pickup>,
/// The despatch of this package.
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<Despatch>,
/// The status of this transport handling unit.
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
}

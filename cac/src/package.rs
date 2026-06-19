#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ReturnableMaterialIndicator")]
    pub returnable_material_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PackageLevelCode")]
    pub package_level_code: Option<cct::Code>,
    #[serde(default, rename = "PackagingTypeCode")]
    pub packaging_type_code: Option<cct::Code>,
    #[serde(default, rename = "PackagingType")]
    pub packaging_type: Vec<cct::Text>,
    #[serde(default, rename = "PackingMaterial")]
    pub packing_material: Vec<cct::Text>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
    #[serde(default, rename = "ContainedPackage")]
    pub contained_package: Vec<Package>,
    #[serde(default, rename = "ContainingTransportEquipment")]
    pub containing_transport_equipment: Option<TransportEquipment>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: Vec<DeliveryUnit>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Delivery>,
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<Pickup>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<Despatch>,
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
}

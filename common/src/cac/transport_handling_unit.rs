#[derive(Debug, Deserialize, Serialize)]
pub struct TransportHandlingUnit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TransportHandlingUnitTypeCode")]
    pub transport_handling_unit_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalPackageQuantity")]
    pub total_package_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<super::cct::TextType>,
    #[serde(default, rename = "ShippingMarks")]
    pub shipping_marks: Vec<super::cct::TextType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HandlingUnitDespatchLine")]
    pub handling_unit_despatch_line: Vec<DespatchLine>,
    #[serde(default, rename = "ActualPackage")]
    pub actual_package: Vec<Package>,
    #[serde(default, rename = "ReceivedHandlingUnitReceiptLine")]
    pub received_handling_unit_receipt_line: Vec<ReceiptLine>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Vec<TransportMeans>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<HazardousGoodsTransit>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default, rename = "FloorSpaceMeasurementDimension")]
    pub floor_space_measurement_dimension: Option<Dimension>,
    #[serde(default, rename = "PalletSpaceMeasurementDimension")]
    pub pallet_space_measurement_dimension: Option<Dimension>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: Vec<CustomsDeclaration>,
    #[serde(default, rename = "ReferencedShipment")]
    pub referenced_shipment: Vec<Shipment>,
    #[serde(default, rename = "Package")]
    pub package: Vec<Package>,
    #[serde(default, rename = "DamageDocumentationAttachment")]
    pub damage_documentation_attachment: Vec<Attachment>,
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: Vec<EnergyConsumptionAllocation>,
}

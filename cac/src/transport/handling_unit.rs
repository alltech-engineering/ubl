#[derive(Debug, Deserialize, Serialize)]
pub struct TransportHandlingUnit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "TransportHandlingUnitTypeCode")]
    pub transport_handling_unit_type_code: Option<cct::Code>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<cct::Code>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<cct::Text>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TotalPackageQuantity")]
    pub total_package_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<cct::Text>,
    #[serde(default, rename = "ShippingMarks")]
    pub shipping_marks: Vec<cct::Text>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
    #[serde(default, rename = "HandlingUnitDespatchLine")]
    pub handling_unit_despatch_line: Vec<crate::DespatchLine>,
    #[serde(default, rename = "ActualPackage")]
    pub actual_package: Vec<crate::Package>,
    #[serde(default, rename = "ReceivedHandlingUnitReceiptLine")]
    pub received_handling_unit_receipt_line: Vec<crate::ReceiptLine>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Vec<TransportMeans>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<crate::HazardousGoodsTransit>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
    #[serde(default, rename = "FloorSpaceMeasurementDimension")]
    pub floor_space_measurement_dimension: Option<crate::Dimension>,
    #[serde(default, rename = "PalletSpaceMeasurementDimension")]
    pub pallet_space_measurement_dimension: Option<crate::Dimension>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Status")]
    pub status: Vec<crate::Status>,
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: Vec<crate::CustomsDeclaration>,
    #[serde(default, rename = "ReferencedShipment")]
    pub referenced_shipment: Vec<crate::Shipment>,
    #[serde(default, rename = "Package")]
    pub package: Vec<crate::Package>,
    #[serde(default, rename = "DamageDocumentationAttachment")]
    pub damage_documentation_attachment: Vec<crate::Attachment>,
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: Vec<crate::EnergyConsumptionAllocation>,
}

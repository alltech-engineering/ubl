#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a uniquely identifiable unit consisting of one or more packages, goods items, or
/// pieces of transport equipment.
///
/// UBL Dictionary Entry Name: `Transport Handling Unit. Details`
///
/// Generated from XSD type `TransportHandlingUnitType`.
pub struct TransportHandlingUnit {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this transport handling unit.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of this transport handling unit.
    #[serde(default, rename = "TransportHandlingUnitTypeCode")]
    pub transport_handling_unit_type_code: Option<cct::Code>,
/// The handling required for this transport handling unit, expressed as a code.
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<cct::Code>,
/// The handling required for this transport handling unit, expressed as text.
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<cct::Text>,
/// An indicator that the materials contained in this transport handling unit are subject to an
/// international regulation concerning the carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// The total number of goods items in this transport handling unit.
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<cct::Quantity>,
/// The total number of packages in this transport handling unit.
    #[serde(default, rename = "TotalPackageQuantity")]
    pub total_package_quantity: Option<cct::Quantity>,
/// Text describing damage associated with this transport handling unit.
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<cct::Text>,
/// Text describing the marks and numbers on this transport handling unit.
    #[serde(default, rename = "ShippingMarks")]
    pub shipping_marks: Vec<cct::Text>,
/// An identifier for use in tracing this transport handling unit, such as the EPC number used in RFID.
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
/// A despatch line associated with this transport handling unit.
    #[serde(default, rename = "HandlingUnitDespatchLine")]
    pub handling_unit_despatch_line: Vec<crate::DespatchLine>,
/// A package contained in this transport handling unit.
    #[serde(default, rename = "ActualPackage")]
    pub actual_package: Vec<crate::Package>,
/// A receipt line associated with this transport handling unit.
    #[serde(default, rename = "ReceivedHandlingUnitReceiptLine")]
    pub received_handling_unit_receipt_line: Vec<crate::ReceiptLine>,
/// A piece of transport equipment associated with this transport handling unit.
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<TransportEquipment>,
/// A means of transport associated with this transport handling unit.
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Vec<TransportMeans>,
/// Transit-related information regarding a type of hazardous goods contained in this transport handling
/// unit.
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<crate::HazardousGoodsTransit>,
/// A measurable dimension (length, mass, weight, or volume) of this transport handling unit.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
/// The minimum required operating temperature of this transport handling unit.
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
/// The maximum allowable operating temperature of this transport handling unit.
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
/// A goods item contained in this transport handling unit.
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
/// The floor space measurement dimension associated with this transport handling unit.
    #[serde(default, rename = "FloorSpaceMeasurementDimension")]
    pub floor_space_measurement_dimension: Option<crate::Dimension>,
/// The pallet space measurement dimension associated to this transport handling unit.
    #[serde(default, rename = "PalletSpaceMeasurementDimension")]
    pub pallet_space_measurement_dimension: Option<crate::Dimension>,
/// A reference to a shipping document associated with this transport handling unit.
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<crate::DocumentReference>,
/// The status of this transport handling unit.
    #[serde(default, rename = "Status")]
    pub status: Vec<crate::Status>,
/// Describes identifiers or references relating to customs procedures.
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: Vec<crate::CustomsDeclaration>,
/// A shipment associated with this transport handling unit.
    #[serde(default, rename = "ReferencedShipment")]
    pub referenced_shipment: Vec<crate::Shipment>,
/// A package contained in this transport handling unit.
    #[serde(default, rename = "Package")]
    pub package: Vec<crate::Package>,
/// An attachment, such as a photo, documenting damage associated with this transport handling unit.
    #[serde(default, rename = "DamageDocumentationAttachment")]
    pub damage_documentation_attachment: Vec<crate::Attachment>,
/// An allocation of energy consumption and associated emissions attributable to the handling or
/// transport of this unit.
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: Vec<crate::EnergyConsumptionAllocation>,
}

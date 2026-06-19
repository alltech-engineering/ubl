use serde::{Deserialize, Serialize};


include!("stage.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class defining an identifiable collection of one or more goods items to be transported between the
/// seller party and the buyer party. This information may be defined within a commercial contract. A
/// shipment can be transported in different consignments (e.g., split for logistical purposes).
///
/// UBL Dictionary Entry Name: `Shipment. Details`
///
/// Generated from XSD type `ShipmentType`.
pub struct Shipment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this shipment.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the priority or level of service required for this shipment.
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: Option<cct::Code>,
/// The handling required for this shipment, expressed as a code.
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<cct::Code>,
/// The handling required for this shipment, expressed as text.
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<cct::Text>,
/// Free-form text pertinent to this shipment, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
/// The total gross weight of a shipment; the weight of the goods plus packaging plus transport
/// equipment.
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
/// The net weight of this shipment, excluding packaging.
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
/// The total net weight of this shipment, excluding packaging and transport equipment.
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<cct::Measure>,
/// The total volume of the goods in this shipment, including packaging.
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
/// The total volume of the goods in this shipment, excluding packaging and transport equipment.
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
/// The total number of goods items in this shipment.
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<cct::Quantity>,
/// The number of pieces of transport handling equipment (pallets, boxes, cases, etc.) in this shipment.
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: Option<cct::Quantity>,
/// The amount covered by insurance for this shipment.
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<cct::Amount>,
/// The total declared value for customs purposes of those goods in this shipment that are subject to
/// the same customs procedure and have the same tariff/statistical heading, country information, and
/// duty regime.
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<cct::Amount>,
/// The value of this shipment, declared by the shipper or his agent solely for the purpose of varying
/// the carrier's level of liability from that provided in the contract of carriage, in case of loss or
/// damage to goods or delayed delivery.
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<cct::Amount>,
/// The value, declared for statistical purposes, of those goods in this shipment that have the same
/// statistical heading.
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<cct::Amount>,
/// The monetary amount that has to be or has been paid as calculated under the applicable trade
/// delivery.
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<cct::Amount>,
/// Special instructions relating to this shipment.
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: Vec<cct::Text>,
/// Delivery instructions relating to this shipment.
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: Vec<cct::Text>,
/// An indicator that the consignment has been split in transit (true) or not (false).
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: Option<udt::Indicator>,
/// The total number of consignments within this shipment.
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
/// A consignment covering this shipment.
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<crate::Consignment>,
/// A goods item included in this shipment.
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
/// A stage in the transport movement of this shipment.
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<ShipmentStage>,
/// The delivery of this shipment.
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<crate::Delivery>>,
/// A transport handling unit associated with this shipment.
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: Vec<crate::TransportHandlingUnit>,
/// The address to which a shipment ought to be returned.
    #[serde(default, rename = "ReturnAddress")]
    pub return_address: Option<crate::Address>,
/// The region in which the goods have been produced or manufactured, according to criteria laid down
/// for the purposes of application of the customs tariff, or of quantitative restrictions, or of any
/// other measure related to trade.
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Option<crate::Address>,
/// The first arrival location of a shipment. This would be a port for sea, an airport for air, a
/// terminal for rail, or a border post for land crossing.
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: Option<crate::Location>,
/// The final exporting location for a shipment. This would be a port for sea, an airport for air, a
/// terminal for rail, or a border post for land crossing.
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: Option<crate::Location>,
/// The country from which the goods were originally exported, without any commercial transaction taking
/// place in intermediate countries.
    #[serde(default, rename = "ExportCountry")]
    pub export_country: Option<crate::Country>,
/// A cost incurred by the shipper in moving goods, by whatever means, from one place to another under
/// the terms of the contract of carriage. In addition to transport costs, this may include such
/// elements as packing, documentation, loading, unloading, and insurance to the extent that they relate
/// to the freight costs.
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
/// One or more Insurance Policies that apply to this Shipment.
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<crate::InsurancePolicy>,
}

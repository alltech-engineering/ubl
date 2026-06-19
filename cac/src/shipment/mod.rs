use serde::{Deserialize, Serialize};


include!("stage.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Shipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: Option<cct::Code>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<cct::Code>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<cct::Text>,
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: Vec<cct::Text>,
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: Vec<cct::Text>,
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<crate::Consignment>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<crate::Delivery>>,
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: Vec<crate::TransportHandlingUnit>,
    #[serde(default, rename = "ReturnAddress")]
    pub return_address: Option<crate::Address>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Option<crate::Address>,
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: Option<crate::Location>,
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: Option<crate::Location>,
    #[serde(default, rename = "ExportCountry")]
    pub export_country: Option<crate::Country>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<crate::InsurancePolicy>,
}

// UBL Delivery aggregates — delivery terms, despatch, shipment.

use crate::cac::address::Address;
use crate::cac::allowance::AllowanceCharge;
use crate::cac::contact::Contact;
use crate::cac::item::Item;
use crate::cac::party::Party;
use crate::cac::period::Period;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub quantity: Option<Quantity>,
    #[serde(default)]
    pub minimum_quantity: Option<MinimumQuantity>,
    #[serde(default)]
    pub maximum_quantity: Option<MaximumQuantity>,
    #[serde(default)]
    pub actual_delivery_date: Option<ActualDeliveryDate>,
    #[serde(default)]
    pub actual_delivery_time: Option<ActualDeliveryTime>,
    #[serde(default)]
    pub latest_delivery_date: Option<LatestDeliveryDate>,
    #[serde(default)]
    pub latest_delivery_time: Option<LatestDeliveryTime>,
    #[serde(default)]
    pub release_id: Option<ReleaseID>,
    #[serde(default)]
    pub tracking_id: Option<TrackingID>,
    #[serde(default)]
    pub delivery_address: Option<Address>,
    #[serde(default)]
    pub requested_delivery_period: Option<Period>,
    #[serde(default)]
    pub promised_delivery_period: Option<Period>,
    #[serde(default)]
    pub estimated_delivery_period: Option<Period>,
    #[serde(default)]
    pub carrier_party: Option<Party>,
    #[serde(default)]
    pub delivery_party: Option<Party>,
    #[serde(default)]
    pub despatch: Option<Despatch>,
    #[serde(default)]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default)]
    pub shipment: Option<Shipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryTerms {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub special_terms: Vec<Text>,
    #[serde(default)]
    pub loss_risk_responsibility_code: Option<Code>,
    #[serde(default)]
    pub loss_risk: Vec<Text>,
    #[serde(default)]
    pub amount: Option<Amount>,
    #[serde(default)]
    pub delivery_location: Option<Location>,
    #[serde(default)]
    pub allowance_charge: Option<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Despatch {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub requested_despatch_date: Option<RequestedDespatchDate>,
    #[serde(default)]
    pub requested_despatch_time: Option<RequestedDespatchTime>,
    #[serde(default)]
    pub estimated_despatch_date: Option<EstimatedDespatchDate>,
    #[serde(default)]
    pub estimated_despatch_time: Option<EstimatedDespatchTime>,
    #[serde(default)]
    pub guaranteed_despatch_date: Option<GuaranteedDespatchDate>,
    #[serde(default)]
    pub guaranteed_despatch_time: Option<GuaranteedDespatchTime>,
    #[serde(default)]
    pub release_id: Option<ReleaseID>,
    #[serde(default)]
    pub actual_despatch_date: Option<ActualDespatchDate>,
    #[serde(default)]
    pub actual_despatch_time: Option<ActualDespatchTime>,
    #[serde(default)]
    pub instructions: Vec<Instructions>,
    #[serde(default)]
    pub despatch_address: Option<Address>,
    #[serde(default)]
    pub despatch_party: Option<Party>,
    #[serde(default)]
    pub contact: Option<Contact>,
    #[serde(default)]
    pub estimated_despatch_period: Option<Period>,
    #[serde(default)]
    pub requested_despatch_period: Option<Period>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shipment {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub shipping_priority_level_code: Option<ShippingPriorityLevelCode>,
    #[serde(default)]
    pub information: Vec<Information>,
    #[serde(default)]
    pub net_net_weight_measure: Option<NetNetWeightMeasure>,
    #[serde(default)]
    pub net_volume_measure: Option<NetVolumeMeasure>,
    #[serde(default)]
    pub insurance_value_amount: Option<InsuranceValueAmount>,
    #[serde(default)]
    pub declared_customs_value_amount: Option<DeclaredCustomsValueAmount>,
    #[serde(default)]
    pub declared_for_carriage_value_amount: Option<DeclaredForCarriageValueAmount>,
    #[serde(default)]
    pub declared_statistics_value_amount: Option<DeclaredStatisticsValueAmount>,
    #[serde(default)]
    pub free_on_board_value_amount: Option<FreeOnBoardValueAmount>,
    #[serde(default)]
    pub handling_code: Option<HandlingCode>,
    #[serde(default)]
    pub handling_instructions: Vec<HandlingInstructions>,
    #[serde(default)]
    pub gross_weight_measure: Option<GrossWeightMeasure>,
    #[serde(default)]
    pub net_weight_measure: Option<NetWeightMeasure>,
    #[serde(default)]
    pub gross_volume_measure: Option<GrossVolumeMeasure>,
    #[serde(default)]
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    #[serde(default)]
    pub total_transport_handling_unit_quantity: Option<TotalTransportHandlingUnitQuantity>,
    #[serde(default)]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default)]
    pub shipment_stage: Vec<ShipmentStage>,
    #[serde(default)]
    pub special_instructions: Vec<SpecialInstructions>,
    #[serde(default)]
    pub delivery_instructions: Vec<DeliveryInstructions>,
    #[serde(default)]
    pub split_consignment_indicator: Option<SplitConsignmentIndicator>,
    #[serde(default)]
    pub consignment_quantity: Option<ConsignmentQuantity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipmentStage {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub transport_mode_code: Option<TransportModeCode>,
    #[serde(default)]
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
    #[serde(default)]
    pub transit_direction_code: Option<TransitDirectionCode>,
    #[serde(default)]
    pub instructions: Vec<Instructions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItem {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub item: Vec<Item>,
    #[serde(default)]
    pub quantity: Option<Quantity>,
}

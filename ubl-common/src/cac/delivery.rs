// UBL Delivery aggregates — delivery terms, despatch, shipment.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::address::Address;
use crate::cac::contact::Contact;
use crate::cac::period::Period;
use crate::cac::party::Party;
use crate::cac::allowance::AllowanceCharge;
use crate::cac::item::Item;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    pub id: Option<ID>,
    pub quantity: Option<Quantity>,
    pub minimum_quantity: Option<MinimumQuantity>,
    pub maximum_quantity: Option<MaximumQuantity>,
    pub actual_delivery_date: Option<ActualDeliveryDate>,
    pub actual_delivery_time: Option<ActualDeliveryTime>,
    pub latest_delivery_date: Option<LatestDeliveryDate>,
    pub latest_delivery_time: Option<LatestDeliveryTime>,
    pub release_id: Option<ReleaseID>,
    pub tracking_id: Option<TrackingID>,
    pub delivery_address: Option<Address>,
    pub requested_delivery_period: Option<Period>,
    pub promised_delivery_period: Option<Period>,
    pub estimated_delivery_period: Option<Period>,
    pub carrier_party: Option<Party>,
    pub delivery_party: Option<Party>,
    pub despatch: Option<Despatch>,
    #[serde(default)]
    pub delivery_terms: Vec<DeliveryTerms>,
    pub shipment: Option<Shipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryTerms {
    pub id: Option<ID>,
    #[serde(default)]
    pub special_terms: Vec<Text>,
    pub loss_risk_responsibility_code: Option<Code>,
    #[serde(default)]
    pub loss_risk: Vec<Text>,
    pub amount: Option<Amount>,
    pub delivery_location: Option<Location>,
    pub allowance_charge: Option<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Despatch {
    pub id: Option<ID>,
    pub requested_despatch_date: Option<RequestedDespatchDate>,
    pub requested_despatch_time: Option<RequestedDespatchTime>,
    pub estimated_despatch_date: Option<EstimatedDespatchDate>,
    pub estimated_despatch_time: Option<EstimatedDespatchTime>,
    pub guaranteed_despatch_date: Option<GuaranteedDespatchDate>,
    pub guaranteed_despatch_time: Option<GuaranteedDespatchTime>,
    pub release_id: Option<ReleaseID>,
    pub actual_despatch_date: Option<ActualDespatchDate>,
    pub actual_despatch_time: Option<ActualDespatchTime>,
    #[serde(default)]
    pub instructions: Vec<Instructions>,
    pub despatch_address: Option<Address>,
    pub despatch_party: Option<Party>,
    pub contact: Option<Contact>,
    pub estimated_despatch_period: Option<Period>,
    pub requested_despatch_period: Option<Period>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shipment {
    pub id: Option<ID>,
    pub shipping_priority_level_code: Option<ShippingPriorityLevelCode>,
    #[serde(default)]
    pub information: Vec<Information>,
    pub net_net_weight_measure: Option<NetNetWeightMeasure>,
    pub net_volume_measure: Option<NetVolumeMeasure>,
    pub insurance_value_amount: Option<InsuranceValueAmount>,
    pub declared_customs_value_amount: Option<DeclaredCustomsValueAmount>,
    pub declared_for_carriage_value_amount: Option<DeclaredForCarriageValueAmount>,
    pub declared_statistics_value_amount: Option<DeclaredStatisticsValueAmount>,
    pub free_on_board_value_amount: Option<FreeOnBoardValueAmount>,
    pub handling_code: Option<HandlingCode>,
    #[serde(default)]
    pub handling_instructions: Vec<HandlingInstructions>,
    pub gross_weight_measure: Option<GrossWeightMeasure>,
    pub net_weight_measure: Option<NetWeightMeasure>,
    pub gross_volume_measure: Option<GrossVolumeMeasure>,
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    pub total_transport_handling_unit_quantity: Option<TotalTransportHandlingUnitQuantity>,
    #[serde(default)]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default)]
    pub shipment_stage: Vec<ShipmentStage>,
    #[serde(default)]
    pub special_instructions: Vec<SpecialInstructions>,
    #[serde(default)]
    pub delivery_instructions: Vec<DeliveryInstructions>,
    pub split_consignment_indicator: Option<SplitConsignmentIndicator>,
    pub consignment_quantity: Option<ConsignmentQuantity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipmentStage {
    pub id: Option<ID>,
    pub transport_mode_code: Option<TransportModeCode>,
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
    pub transit_direction_code: Option<TransitDirectionCode>,
    #[serde(default)]
    pub instructions: Vec<Instructions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItem {
    pub id: Option<ID>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub item: Vec<Item>,
    pub quantity: Option<Quantity>,
}

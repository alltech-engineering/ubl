// UBL Delivery aggregates — delivery terms, delivery info.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::address::Address;
use crate::cac::party::Party;
use crate::cac::period::Period;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delivery {
    pub id: Option<ID>,
    pub quantity: Option<Quantity>,
    pub minimum_quantity: Option<MinimumQuantity>,
    pub maximum_quantity: Option<MaximumQuantity>,
    pub actual_delivery_date: Option<ActualDeliveryDate>,
    pub actual_delivery_time: Option<ActualDeliveryTime>,
    pub latest_delivery_date: Option<LatestDeliveryDate>,
    pub latest_delivery_time: Option<Time>,
    pub tracking_id: Option<TrackingID>,
    pub delivery_address: Option<Address>,
    pub alternative_delivery_location: Option<Location>,
    pub requested_delivery_period: Option<Period>,
    pub promised_delivery_period: Option<Period>,
    pub estimated_delivery_period: Option<Period>,
    pub carrier_party: Option<Party>,
    pub delivery_party: Option<Party>,
    pub despatch: Option<Despatch>,
    pub delivery_terms: Vec<DeliveryTerms>,
    pub shipment: Option<Shipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryTerms {
    pub id: Option<ID>,
    pub special_terms: Vec<Text>,
    pub loss_risk_responsibility_code: Option<Code>,
    pub loss_risk: Vec<Text>,
    pub amount: Option<Amount>,
    pub delivery_location: Option<Location>,
    pub allowance_charge: Option<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Despatch {
    pub id: Option<ID>,
    pub actual_despatch_date: Option<ActualDespatchDate>,
    pub actual_despatch_time: Option<ActualDespatchTime>,
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
    pub handling_code: Option<HandlingCode>,
    pub handling_instructions: Vec<HandlingInstructions>,
    pub gross_weight_measure: Option<GrossWeightMeasure>,
    pub net_weight_measure: Option<NetWeightMeasure>,
    pub gross_volume_measure: Option<GrossVolumeMeasure>,
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    pub total_transport_handling_unit_quantity: Option<TotalTransportHandlingUnitQuantity>,
    pub shipment_stage: Vec<ShipmentStage>,
    pub goods_item: Vec<GoodsItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipmentStage {
    pub id: Option<ID>,
    pub transport_mode_code: Option<TransportModeCode>,
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItem {
    pub id: Option<ID>,
    pub description: Option<Description>,
    pub item: Vec<Item>,
}

use crate::cac::allowance::AllowanceCharge;
use crate::cac::contact::Contact;
use crate::cac::item::Item;

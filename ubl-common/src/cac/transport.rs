// UBL Transport aggregates — equipment, handling units, means, and services.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::address::Address;
use crate::cac::party::Party;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEquipment {
    pub id: Option<ID>,
    pub transport_equipment_type_code: Option<TransportEquipmentTypeCode>,
    pub description: Vec<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportHandlingUnit {
    pub id: Option<ID>,
    pub transport_handling_unit_type_code: Option<TransportHandlingUnitTypeCode>,
    pub handling_code: Option<HandlingCode>,
    pub handling_instructions: Vec<HandlingInstructions>,
    pub hazardous_risk_indicator: Option<HazardousRiskIndicator>,
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    pub total_package_quantity: Option<TotalPackagesQuantity>,
    pub damage_remarks: Vec<DamageRemarks>,
    pub shipping_marks: Vec<ShippingMarks>,
    pub temperature: Vec<Temperature>,
    pub goods_item: Vec<GoodsItem>,
    pub transport_equipment: Vec<TransportEquipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportMeans {
    pub journey_id: Option<ID>,
    pub registration_nationality_id: Option<RegistrationNationalityID>,
    pub registration_nationality: Vec<Text>,
    pub direction_code: Option<DirectionCode>,
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
    pub trade_service_code: Option<TradeServiceCode>,
    pub stowage_value_text: Vec<Text>,
    pub stowage_value_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationService {
    pub transport_service_code: TransportServiceCode,
    pub tariff_class_code: Option<TariffClassCode>,
    pub priority: Option<Text>,
    pub freight_rate_class_code: Option<Code>,
    pub transportation_service_description: Vec<Text>,
    pub transportation_service_details_uri: Option<Text>,
    pub nomination_date: Option<Date>,
    pub nomination_time: Option<Time>,
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub attribute_id: Option<AttributeID>,
    pub measure: TemperatureMeasure,
    pub description: Vec<Description>,
}

// Forward declarations for cross-module types
use crate::cac::delivery::GoodsItem;

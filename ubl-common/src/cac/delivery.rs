// Delivery — UBL CAC aggregate
// Delivery information for a shipment.
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Delivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery_date: Option<ActualDeliveryDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_delivery_time: Option<ActualDeliveryTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_date: Option<LatestDeliveryDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<LatestDeliveryTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_period: Option<Period>,
}
use super::period::Period;
use super::address::Address;

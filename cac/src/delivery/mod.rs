use serde::{Deserialize, Serialize};

pub type DeliveryLocation = crate::Location;
pub type DeliveryParty = crate::Party;
pub type DeliveryPeriod = crate::Period;
pub type DeliveryTransportEvent = crate::TransportEvent;
pub type DeliveryTransportationService = crate::TransportationService;

include!("unit.rs");
include!("terms.rs");
include!("channel.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Delivery {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ActualDeliveryDate")]
    pub actual_delivery_date: Option<udt::DateTime>,
    #[serde(default, rename = "ActualDeliveryTime")]
    pub actual_delivery_time: Option<udt::DateTime>,
    #[serde(default, rename = "LatestDeliveryDate")]
    pub latest_delivery_date: Option<udt::DateTime>,
    #[serde(default, rename = "LatestDeliveryTime")]
    pub latest_delivery_time: Option<udt::DateTime>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<cct::Identifier>,
    #[serde(default, rename = "TrackingID")]
    pub tracking_id: Option<cct::Identifier>,
    #[serde(default, rename = "DeliveryAddress")]
    pub delivery_address: Option<crate::Address>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<crate::Location>,
    #[serde(default, rename = "AlternativeDeliveryLocation")]
    pub alternative_delivery_location: Option<crate::Location>,
    #[serde(default, rename = "RequestedDeliveryPeriod")]
    pub requested_delivery_period: Option<crate::Period>,
    #[serde(default, rename = "PromisedDeliveryPeriod")]
    pub promised_delivery_period: Option<crate::Period>,
    #[serde(default, rename = "EstimatedDeliveryPeriod")]
    pub estimated_delivery_period: Option<crate::Period>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<crate::Party>,
    #[serde(default, rename = "DeliveryParty")]
    pub delivery_party: Option<crate::Party>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<crate::Party>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<crate::Despatch>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default, rename = "MinimumDeliveryUnit")]
    pub minimum_delivery_unit: Option<DeliveryUnit>,
    #[serde(default, rename = "MaximumDeliveryUnit")]
    pub maximum_delivery_unit: Option<DeliveryUnit>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<crate::Shipment>,
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: Vec<crate::FuelConsumption>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "DeliveryNoteLineReference")]
    pub delivery_note_line_reference: Vec<crate::LineReference>,
}

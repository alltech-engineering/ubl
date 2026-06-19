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
/// A class to describe a delivery.
///
/// UBL Dictionary Entry Name: `Delivery. Details`
///
/// Generated from XSD type `DeliveryType`.
pub struct Delivery {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this delivery.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The quantity of items, child consignments, shipments in this delivery.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The minimum quantity of items, child consignments, shipments in this delivery.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The maximum quantity of items, child consignments, shipments in this delivery.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// The actual date of delivery.
    #[serde(default, rename = "ActualDeliveryDate")]
    pub actual_delivery_date: Option<udt::DateTime>,
/// The actual time of delivery.
    #[serde(default, rename = "ActualDeliveryTime")]
    pub actual_delivery_time: Option<udt::DateTime>,
/// The latest date of delivery allowed by the buyer.
    #[serde(default, rename = "LatestDeliveryDate")]
    pub latest_delivery_date: Option<udt::DateTime>,
/// The latest time of delivery allowed by the buyer.
    #[serde(default, rename = "LatestDeliveryTime")]
    pub latest_delivery_time: Option<udt::DateTime>,
/// An identifier used for approval of access to delivery locations (e.g., port terminals).
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<cct::Identifier>,
/// The delivery Tracking ID (for transport tracking).
    #[serde(default, rename = "TrackingID")]
    pub tracking_id: Option<cct::Identifier>,
/// The delivery address.
    #[serde(default, rename = "DeliveryAddress")]
    pub delivery_address: Option<crate::Address>,
/// The delivery location.
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<crate::Location>,
/// An alternative delivery location.
    #[serde(default, rename = "AlternativeDeliveryLocation")]
    pub alternative_delivery_location: Option<crate::Location>,
/// The period requested for delivery.
    #[serde(default, rename = "RequestedDeliveryPeriod")]
    pub requested_delivery_period: Option<crate::Period>,
/// The period promised for delivery.
    #[serde(default, rename = "PromisedDeliveryPeriod")]
    pub promised_delivery_period: Option<crate::Period>,
/// The period estimated for delivery.
    #[serde(default, rename = "EstimatedDeliveryPeriod")]
    pub estimated_delivery_period: Option<crate::Period>,
/// The Party who provides the transport of goods between named points.
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<crate::Party>,
/// The Party who receives the goods.
    #[serde(default, rename = "DeliveryParty")]
    pub delivery_party: Option<crate::Party>,
/// The Party who is notified of this Delivery.
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<crate::Party>,
/// The despatch (pickup) associated with this delivery.
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<crate::Despatch>,
/// Terms and conditions relating to the delivery.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<DeliveryTerms>,
/// The minimum delivery unit for this delivery.
    #[serde(default, rename = "MinimumDeliveryUnit")]
    pub minimum_delivery_unit: Option<DeliveryUnit>,
/// The maximum delivery unit for this delivery.
    #[serde(default, rename = "MaximumDeliveryUnit")]
    pub maximum_delivery_unit: Option<DeliveryUnit>,
/// The shipment being delivered.
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<crate::Shipment>,
/// One or more fuel consumptions of this delivery.
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: Vec<crate::FuelConsumption>,
/// A reference to a Delivery Note associated with this Delivery.
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<crate::DocumentReference>,
/// A reference to a Delivery Note Line associated with this Delivery.
    #[serde(default, rename = "DeliveryNoteLineReference")]
    pub delivery_note_line_reference: Vec<crate::LineReference>,
}

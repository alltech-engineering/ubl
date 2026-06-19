use serde::{Deserialize, Serialize};

pub type TransportServiceProviderParty = crate::Party;

include!("execution_terms.rs");
include!("equipment.rs");
include!("handling_unit.rs");
include!("schedule.rs");
include!("event.rs");
include!("equipment_seal.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a particular vehicle or vessel used for the conveyance of goods or persons.
///
/// UBL Dictionary Entry Name: `Transport Means. Details`
///
/// Generated from XSD type `TransportMeansType`.
pub struct TransportMeans {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the regular service schedule of this means of transport.
    #[serde(default, rename = "JourneyID")]
    pub journey_id: Option<cct::Identifier>,
/// An identifier for the country in which this means of transport is registered.
    #[serde(default, rename = "RegistrationNationalityID")]
    pub registration_nationality_id: Option<cct::Identifier>,
/// Text describing the country in which this means of transport is registered.
    #[serde(default, rename = "RegistrationNationality")]
    pub registration_nationality: Vec<cct::Text>,
/// A code signifying the direction of this means of transport.
    #[serde(default, rename = "DirectionCode")]
    pub direction_code: Option<cct::Code>,
/// A code signifying the type of this means of transport (truck, vessel, etc.).
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: Option<cct::Code>,
/// A code signifying the service regularly provided by the carrier operating this means of transport.
    #[serde(default, rename = "TradeServiceCode")]
    pub trade_service_code: Option<cct::Code>,
/// The location within the means of transport where goods are to be or have been stowed.
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<crate::Stowage>,
/// An aircraft used for transport.
    #[serde(default, rename = "AirTransport")]
    pub air_transport: Option<crate::AirTransport>,
/// A vehicle used for road transport.
    #[serde(default, rename = "RoadTransport")]
    pub road_transport: Option<crate::RoadTransport>,
/// Equipment used for rail transport.
    #[serde(default, rename = "RailTransport")]
    pub rail_transport: Option<crate::RailTransport>,
/// A vessel used for transport by water (not only by sea).
    #[serde(default, rename = "MaritimeTransport")]
    pub maritime_transport: Option<crate::MaritimeTransport>,
/// The Party who owns these Means of Transport.
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<crate::Party>,
/// A measurable dimension (length, mass, weight, or volume) of this means of transport.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
}

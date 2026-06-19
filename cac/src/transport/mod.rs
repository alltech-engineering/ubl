use serde::{Deserialize, Serialize};

pub type TransportServiceProviderParty = crate::Party;

include!("execution_terms.rs");
include!("equipment.rs");
include!("handling_unit.rs");
include!("schedule.rs");
include!("event.rs");
include!("equipment_seal.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportMeans {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "JourneyID")]
    pub journey_id: Option<cct::Identifier>,
    #[serde(default, rename = "RegistrationNationalityID")]
    pub registration_nationality_id: Option<cct::Identifier>,
    #[serde(default, rename = "RegistrationNationality")]
    pub registration_nationality: Vec<cct::Text>,
    #[serde(default, rename = "DirectionCode")]
    pub direction_code: Option<cct::Code>,
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: Option<cct::Code>,
    #[serde(default, rename = "TradeServiceCode")]
    pub trade_service_code: Option<cct::Code>,
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<crate::Stowage>,
    #[serde(default, rename = "AirTransport")]
    pub air_transport: Option<crate::AirTransport>,
    #[serde(default, rename = "RoadTransport")]
    pub road_transport: Option<crate::RoadTransport>,
    #[serde(default, rename = "RailTransport")]
    pub rail_transport: Option<crate::RailTransport>,
    #[serde(default, rename = "MaritimeTransport")]
    pub maritime_transport: Option<crate::MaritimeTransport>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<crate::Party>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
}

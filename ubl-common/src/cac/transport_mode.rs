// UBL 2.5 CAC Tier 3-4: Transport Mode Types
// Maritime, Road, Rail, and Air transport details
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── MaritimeTransport ───────────────────────────────────────────────
// XSD: MaritimeTransportType
// Details of transport by sea

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaritimeTransport {
    pub vessel_id: Option<String>,
    pub vessel_name: Option<String>,
    pub radio_call_sign_id: Option<String>,
    pub mmsi_registration_id: Option<String>,
    pub ships_requirements: Vec<String>,
    pub gross_tonnage_measure: Option<f64>,
    pub net_tonnage_measure: Option<f64>,
    pub segregated_ballast_measure: Option<f64>,
    pub ship_configuration_code: Option<String>,
    pub inf_ship_class_code: Option<String>,
    pub antenna_locus: Vec<String>,
    // CAC: registry_certificate_document_reference: Option<DocumentReference>
    // CAC: registry_port_location: Option<Location>
    // CAC: vessel_dynamics: Option<VesselDynamics>
}

// ─── RoadTransport ───────────────────────────────────────────────────
// XSD: RoadTransportType
// Details of transport by road

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadTransport {
    pub license_plate_id: Option<String>,
    pub trailer_license_plate_id: Option<String>,
}

// ─── RailTransport ───────────────────────────────────────────────────
// XSD: RailTransportType
// Details of transport by rail

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RailTransport {
    pub train_id: Option<String>,
    pub rail_car_id: Option<String>,
}

// ─── AirTransport ────────────────────────────────────────────────────
// XSD: AirTransportType
// Details of transport by air

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirTransport {
    pub aircraft_id: Option<String>,
}

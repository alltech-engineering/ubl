// UBL 2.5 CAC Tier 3-4: Stowage, Transport Equipment Seal
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── Stowage ─────────────────────────────────────────────────────────
// XSD: StowageType
// A location on board a means of transport where goods are stowed

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stowage {
    pub location_id: Option<String>,
    pub location: Vec<String>,
    // CAC: measurement_dimension: Vec<Dimension>
}

// ─── TransportEquipmentSeal ──────────────────────────────────────────
// XSD: TransportEquipmentSealType
// A seal applied to transport equipment for security

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEquipmentSeal {
    pub id: Option<String>,
    pub seal_issuer_type_code: Option<String>,
    pub condition: Option<String>,
    pub seal_status_code: Option<String>,
    pub sealing_party_type: Option<String>,
}

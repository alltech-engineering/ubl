// UBL 2.5 CAC Tier 3-4: Hazardous Goods Transit, Secondary Hazard
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── HazardousGoodsTransit ───────────────────────────────────────────
// XSD: HazardousGoodsTransitType
// Hazardous goods information for the transit stage

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardousGoodsTransit {
    #[serde(default)]
    pub transport_emergency_card_code: Option<String>,
    #[serde(default)]
    pub packing_criteria_code: Option<String>,
    #[serde(default)]
    pub hazardous_regulation_code: Option<String>,
    #[serde(default)]
    pub inhalation_toxicity_zone_code: Option<String>,
    #[serde(default)]
    pub transport_authorization_code: Option<String>,
    #[serde(default)]
    pub transit_description: Vec<String>,
    // CAC: maximum_temperature: Option<Temperature>
    // CAC: minimum_temperature: Option<Temperature>
}

// ─── SecondaryHazard ─────────────────────────────────────────────────
// XSD: SecondaryHazardType
// A secondary hazard associated with a hazardous item

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecondaryHazard {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub placard_notation: Option<String>,
    #[serde(default)]
    pub placard_endorsement: Option<String>,
    #[serde(default)]
    pub emergency_procedures_code: Option<String>,
    #[serde(default)]
    pub extension: Vec<String>,
}

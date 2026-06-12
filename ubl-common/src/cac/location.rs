// UBL 2.5 CAC Tier 3: Location, Dimension, Temperature, Country
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── Location ────────────────────────────────────────────────────────
// XSD: LocationType
// A geographic or physical location

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub id: Option<String>,
    pub description: Vec<String>,
    pub conditions: Vec<String>,
    pub country_subentity: Option<String>,
    pub country_subentity_code: Option<String>,
    pub location_type_code: Option<String>,
    pub information_uri: Option<String>,
    pub name: Option<String>,
    // CAC: validity_period: Vec<Period>
    // CAC: address: Option<Address>
    // CAC: storage: Option<Storage>
    // CAC: subsidiary_location: Vec<Location>
    // CAC: location_coordinate: Vec<LocationCoordinate>
}

// ─── Dimension ───────────────────────────────────────────────────────
// XSD: DimensionType
// A measurable dimension (length, width, height, weight, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub attribute_id: String, // 1..1 required
    pub measure: Option<f64>,
    pub description: Vec<String>,
    pub minimum_measure: Option<f64>,
    pub maximum_measure: Option<f64>,
}

// ─── Temperature ─────────────────────────────────────────────────────
// XSD: TemperatureType
// A temperature measurement (ambient, operating, transport, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub attribute_id: Option<String>,
    pub measure: Option<f64>,
    pub measure_code: Option<String>,
    pub description: Vec<String>,
}

// ─── Country ─────────────────────────────────────────────────────────
// XSD: CountryType
// A country (used in addresses, trade, customs)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub identification_code: Option<String>,
    pub name: Option<String>,
}

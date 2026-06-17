// UBL 2.5 CAC Tier 3-4: Customs Declaration, Location Coordinate, Classification Category
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── CustomsDeclaration ──────────────────────────────────────────────
// XSD: CustomsDeclarationType
// A customs declaration for import, export, or transit

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomsDeclaration {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub function_code: Option<String>,
    // CAC: validity_period: Option<Period>
    // CAC: applicable_territory_address: Option<Address>
    // CAC: shipment: Option<Shipment>
    // CAC: customs_exit_office_location: Option<Location>
    // CAC: issuer_party: Option<Party>
    // CAC: consignor_party: Option<Party>
    // CAC: consignee_party: Option<Party>
    // CAC: freight_forwarder_party: Option<Party>
    // CAC: customs_party: Option<Party>
    // CAC: previous_customs_declaration: Option<CustomsDeclaration>
    // CAC: additional_document_reference: Vec<DocumentReference>
}

// ─── LocationCoordinate ──────────────────────────────────────────────
// XSD: LocationCoordinateType
// Geographic coordinates for a location

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationCoordinate {
    #[serde(default)]
    pub coordinate_system_code: Option<String>,
    #[serde(default)]
    pub latitude_degrees_measure: Option<f64>,
    #[serde(default)]
    pub latitude_minutes_measure: Option<f64>,
    #[serde(default)]
    pub latitude_direction_code: Option<String>,
    #[serde(default)]
    pub longitude_degrees_measure: Option<f64>,
    #[serde(default)]
    pub longitude_minutes_measure: Option<f64>,
    #[serde(default)]
    pub longitude_direction_code: Option<String>,
    #[serde(default)]
    pub altitude_measure: Option<f64>,
}

// ─── ClassificationCategory ──────────────────────────────────────────
// XSD: ClassificationCategoryType
// A category within a classification scheme

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationCategory {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub code_value: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    // CAC: categorizes_classification_category: Vec<ClassificationCategory>
}

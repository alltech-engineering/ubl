// UBL 2.5 CAC Tier 3-4: Logistics — Package, Delivery Unit, Despatch, Pickup
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── Package ─────────────────────────────────────────────────────────
// XSD: PackageType
// A package containing goods items

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub returnable_material_indicator: Option<bool>,
    #[serde(default)]
    pub package_level_code: Option<String>,
    #[serde(default)]
    pub packaging_type_code: Option<String>,
    #[serde(default)]
    pub packaging_type: Vec<String>,
    #[serde(default)]
    pub packing_material: Vec<String>,
    #[serde(default)]
    pub trace_id: Option<String>,
    // CAC: contained_package: Vec<Package>
    // CAC: containing_transport_equipment: Option<TransportEquipment>
    // CAC: goods_item: Vec<GoodsItem>
    // CAC: measurement_dimension: Vec<Dimension>
    // CAC: delivery_unit: Option<DeliveryUnit>
    // CAC: delivery: Option<Delivery>
    // CAC: pickup: Option<Pickup>
    // CAC: despatch: Option<Despatch>
    // CAC: status: Vec<Status>
}

// ─── GoodsItemContainer ─────────────────────────────────────────────
// XSD: GoodsItemContainerType
// A container for a goods item within transport equipment

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItemContainer {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub quantity: Option<f64>,
    // CAC: transport_equipment: Vec<TransportEquipment>
}

// ─── DeliveryUnit ────────────────────────────────────────────────────
// XSD: DeliveryUnitType
// A unit in which a delivery is measured

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryUnit {
    #[serde(default)]
    pub batch_quantity: Option<f64>,
    #[serde(default)]
    pub consumer_unit_quantity: Option<f64>,
    #[serde(default)]
    pub hazardous_risk_indicator: Option<bool>,
}

// ─── Despatch ────────────────────────────────────────────────────────
// XSD: DespatchType
// The despatch of goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Despatch {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub requested_despatch_date: Option<String>,
    #[serde(default)]
    pub requested_despatch_time: Option<String>,
    #[serde(default)]
    pub estimated_despatch_date: Option<String>,
    #[serde(default)]
    pub estimated_despatch_time: Option<String>,
    #[serde(default)]
    pub actual_despatch_date: Option<String>,
    #[serde(default)]
    pub actual_despatch_time: Option<String>,
    #[serde(default)]
    pub guaranteed_despatch_date: Option<String>,
    #[serde(default)]
    pub guaranteed_despatch_time: Option<String>,
    #[serde(default)]
    pub release_id: Option<String>,
    #[serde(default)]
    pub instructions: Vec<String>,
    // CAC: despatch_address: Option<Address>
    // CAC: despatch_location: Option<Location>
    // CAC: despatch_party: Option<Party>
    // CAC: carrier_party: Option<Party>
    // CAC: notify_party: Vec<Party>
    // CAC: responsible_party: Option<Party>
    // CAC: contact: Option<Contact>
    // CAC: estimated/requested_despatch_period: Option<Period>
}

// ─── Pickup ──────────────────────────────────────────────────────────
// XSD: PickupType
// The pickup of goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pickup {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub actual_pickup_date: Option<String>,
    #[serde(default)]
    pub actual_pickup_time: Option<String>,
    #[serde(default)]
    pub earliest_pickup_date: Option<String>,
    #[serde(default)]
    pub earliest_pickup_time: Option<String>,
    #[serde(default)]
    pub latest_pickup_date: Option<String>,
    #[serde(default)]
    pub latest_pickup_time: Option<String>,
    // CAC: pickup_location: Option<Location>
    // CAC: pickup_party: Option<Party>
}

// UBL 2.5 CAC Tier 3-4: Logistics — Package, Delivery Unit, Despatch, Pickup
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── Package ─────────────────────────────────────────────────────────
// XSD: PackageType
// A package containing goods items

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub id: Option<String>,
    pub quantity: Option<f64>,
    pub returnable_material_indicator: Option<bool>,
    pub package_level_code: Option<String>,
    pub packaging_type_code: Option<String>,
    pub packaging_type: Vec<String>,
    pub packing_material: Vec<String>,
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
    pub id: Option<String>,
    pub quantity: Option<f64>,
    // CAC: transport_equipment: Vec<TransportEquipment>
}

// ─── DeliveryUnit ────────────────────────────────────────────────────
// XSD: DeliveryUnitType
// A unit in which a delivery is measured

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryUnit {
    pub batch_quantity: Option<f64>,
    pub consumer_unit_quantity: Option<f64>,
    pub hazardous_risk_indicator: Option<bool>,
}

// ─── Despatch ────────────────────────────────────────────────────────
// XSD: DespatchType
// The despatch of goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Despatch {
    pub id: Option<String>,
    pub requested_despatch_date: Option<String>,
    pub requested_despatch_time: Option<String>,
    pub estimated_despatch_date: Option<String>,
    pub estimated_despatch_time: Option<String>,
    pub actual_despatch_date: Option<String>,
    pub actual_despatch_time: Option<String>,
    pub guaranteed_despatch_date: Option<String>,
    pub guaranteed_despatch_time: Option<String>,
    pub release_id: Option<String>,
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
    pub id: Option<String>,
    pub actual_pickup_date: Option<String>,
    pub actual_pickup_time: Option<String>,
    pub earliest_pickup_date: Option<String>,
    pub earliest_pickup_time: Option<String>,
    pub latest_pickup_date: Option<String>,
    pub latest_pickup_time: Option<String>,
    // CAC: pickup_location: Option<Location>
    // CAC: pickup_party: Option<Party>
}

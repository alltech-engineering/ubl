// UBL 2.5 CAC Tier 3-4: Goods Item, Hazardous Item, Classification, Item Property
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── GoodsItem ───────────────────────────────────────────────────────
// XSD: GoodsItemType
// A separately identifiable quantity of products for transport

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub sequence_number_id: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub hazardous_risk_indicator: Option<bool>,
    #[serde(default)]
    pub declared_customs_value_amount: Option<f64>,
    #[serde(default)]
    pub declared_for_carriage_value_amount: Option<f64>,
    #[serde(default)]
    pub declared_statistics_value_amount: Option<f64>,
    #[serde(default)]
    pub free_on_board_value_amount: Option<f64>,
    #[serde(default)]
    pub insurance_value_amount: Option<f64>,
    #[serde(default)]
    pub value_amount: Option<f64>,
    #[serde(default)]
    pub gross_weight_measure: Option<f64>,
    #[serde(default)]
    pub net_weight_measure: Option<f64>,
    #[serde(default)]
    pub net_net_weight_measure: Option<f64>,
    #[serde(default)]
    pub chargeable_weight_measure: Option<f64>,
    #[serde(default)]
    pub gross_volume_measure: Option<f64>,
    #[serde(default)]
    pub net_volume_measure: Option<f64>,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub preference_criterion_code: Option<String>,
    #[serde(default)]
    pub required_customs_id: Option<String>,
    #[serde(default)]
    pub customs_status_code: Option<String>,
    #[serde(default)]
    pub customs_procedure_code: Option<String>,
    #[serde(default)]
    pub customs_tariff_quantity: Option<f64>,
    #[serde(default)]
    pub customs_import_classified_indicator: Option<bool>,
    #[serde(default)]
    pub chargeable_quantity: Option<f64>,
    #[serde(default)]
    pub returnable_quantity: Option<f64>,
    #[serde(default)]
    pub trace_id: Option<String>,
    // CAC refs: Item, GoodsItemContainer, AllowanceCharge, InvoiceLine,
    // OrderLineReference, DespatchLineReference, ReceiptLineReference,
    // Temperature, ContainedGoodsItem, OriginAddress, Delivery, Pickup,
    // Despatch, BondedWarehouseLocation, Dimension, ContainingPackage,
    // ShipmentDocumentReference, AdditionalDocumentReference,
    // MinimumTemperature, MaximumTemperature, InsurancePolicy,
    // EnergyConsumptionAllocation
}

// ─── HazardousItem ───────────────────────────────────────────────────
// XSD: HazardousItemType
// Hazardous goods information for transport

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardousItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub placard_notation: Option<String>,
    #[serde(default)]
    pub placard_endorsement: Option<String>,
    #[serde(default)]
    pub additional_information: Vec<String>,
    #[serde(default)]
    pub undg_code: Option<String>,
    #[serde(default)]
    pub un_packing_group_code: Option<String>,
    #[serde(default)]
    pub un_packing_group: Vec<String>,
    #[serde(default)]
    pub emergency_procedures_code: Option<String>,
    #[serde(default)]
    pub medical_first_aid_guide_code: Option<String>,
    #[serde(default)]
    pub tunnel_restriction_code: Option<String>,
    #[serde(default)]
    pub maritime_pollutant_code: Option<String>,
    #[serde(default)]
    pub technical_name: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub proper_shipping_name: Option<String>,
    #[serde(default)]
    pub hazardous_category_code: Option<String>,
    #[serde(default)]
    pub upper_orange_hazard_placard_id: Option<String>,
    #[serde(default)]
    pub lower_orange_hazard_placard_id: Option<String>,
    #[serde(default)]
    pub marking_id: Option<String>,
    #[serde(default)]
    pub hazard_class_id: Option<String>,
    #[serde(default)]
    pub hazardous_type_code: Option<String>,
    #[serde(default)]
    pub packaging_danger_level_code: Option<String>,
    #[serde(default)]
    pub gross_weight_measure: Option<f64>,
    #[serde(default)]
    pub net_weight_measure: Option<f64>,
    #[serde(default)]
    pub net_volume_measure: Option<f64>,
    #[serde(default)]
    pub quantity: Option<f64>,
    // CAC: ContactParty, SecondaryHazard, HazardousGoodsTransit,
    // EmergencyTemperature, FlashpointTemperature, AdditionalTemperature,
    // PositionOnBoardStowage, RadioactiveMaterial, Package
}

// ─── ClassificationScheme ────────────────────────────────────────────
// XSD: ClassificationSchemeType
// A classification system (e.g., UNSPSC, CPV, NAICS)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationScheme {
    pub id: String, // 1..1 required
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub last_revision_date: Option<String>,
    #[serde(default)]
    pub last_revision_time: Option<String>,
    #[serde(default)]
    pub note: Vec<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub agency_id: Option<String>,
    #[serde(default)]
    pub agency_name: Option<String>,
    #[serde(default)]
    pub version_id: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub scheme_uri: Option<String>,
    #[serde(default)]
    pub language_id: Option<String>,
    // CAC: ClassificationCategory (1..n required)
}

// ─── CommodityClassification ─────────────────────────────────────────
// XSD: CommodityClassificationType
// Classification of a commodity/item

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommodityClassification {
    #[serde(default)]
    pub nature_code: Option<String>,
    #[serde(default)]
    pub cargo_type_code: Option<String>,
    #[serde(default)]
    pub commodity_code: Option<String>,
    #[serde(default)]
    pub item_classification_code: Option<String>,
}

// ─── ItemProperty ────────────────────────────────────────────────────
// XSD: ItemPropertyType
// A property or characteristic of an item

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemProperty {
    #[serde(default)]
    pub id: Option<String>,
    pub name: String, // 1..1 required
    #[serde(default)]
    pub name_code: Option<String>,
    #[serde(default)]
    pub test_method: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub value_quantity: Option<f64>,
    #[serde(default)]
    pub value_qualifier: Vec<String>,
    #[serde(default)]
    pub importance_code: Option<String>,
    #[serde(default)]
    pub list_value: Vec<String>,
    // CAC: UsabilityPeriod, ItemPropertyGroup, RangeDimension,
    // ItemPropertyRange, StandardPropertyIdentification, SubItemProperty
}

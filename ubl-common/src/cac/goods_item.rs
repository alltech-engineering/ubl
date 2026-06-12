// UBL 2.5 CAC Tier 3-4: Goods Item, Hazardous Item, Classification, Item Property
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── GoodsItem ───────────────────────────────────────────────────────
// XSD: GoodsItemType
// A separately identifiable quantity of products for transport

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItem {
    pub id: Option<String>,
    pub sequence_number_id: Option<String>,
    pub description: Vec<String>,
    pub hazardous_risk_indicator: Option<bool>,
    pub declared_customs_value_amount: Option<f64>,
    pub declared_for_carriage_value_amount: Option<f64>,
    pub declared_statistics_value_amount: Option<f64>,
    pub free_on_board_value_amount: Option<f64>,
    pub insurance_value_amount: Option<f64>,
    pub value_amount: Option<f64>,
    pub gross_weight_measure: Option<f64>,
    pub net_weight_measure: Option<f64>,
    pub net_net_weight_measure: Option<f64>,
    pub chargeable_weight_measure: Option<f64>,
    pub gross_volume_measure: Option<f64>,
    pub net_volume_measure: Option<f64>,
    pub quantity: Option<f64>,
    pub preference_criterion_code: Option<String>,
    pub required_customs_id: Option<String>,
    pub customs_status_code: Option<String>,
    pub customs_procedure_code: Option<String>,
    pub customs_tariff_quantity: Option<f64>,
    pub customs_import_classified_indicator: Option<bool>,
    pub chargeable_quantity: Option<f64>,
    pub returnable_quantity: Option<f64>,
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
    pub id: Option<String>,
    pub placard_notation: Option<String>,
    pub placard_endorsement: Option<String>,
    pub additional_information: Vec<String>,
    pub undg_code: Option<String>,
    pub un_packing_group_code: Option<String>,
    pub un_packing_group: Vec<String>,
    pub emergency_procedures_code: Option<String>,
    pub medical_first_aid_guide_code: Option<String>,
    pub tunnel_restriction_code: Option<String>,
    pub maritime_pollutant_code: Option<String>,
    pub technical_name: Option<String>,
    pub category_name: Option<String>,
    pub proper_shipping_name: Option<String>,
    pub hazardous_category_code: Option<String>,
    pub upper_orange_hazard_placard_id: Option<String>,
    pub lower_orange_hazard_placard_id: Option<String>,
    pub marking_id: Option<String>,
    pub hazard_class_id: Option<String>,
    pub hazardous_type_code: Option<String>,
    pub packaging_danger_level_code: Option<String>,
    pub gross_weight_measure: Option<f64>,
    pub net_weight_measure: Option<f64>,
    pub net_volume_measure: Option<f64>,
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
    pub uuid: Option<String>,
    pub last_revision_date: Option<String>,
    pub last_revision_time: Option<String>,
    pub note: Vec<String>,
    pub name: Option<String>,
    pub description: Vec<String>,
    pub agency_id: Option<String>,
    pub agency_name: Option<String>,
    pub version_id: Option<String>,
    pub uri: Option<String>,
    pub scheme_uri: Option<String>,
    pub language_id: Option<String>,
    // CAC: ClassificationCategory (1..n required)
}

// ─── CommodityClassification ─────────────────────────────────────────
// XSD: CommodityClassificationType
// Classification of a commodity/item

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommodityClassification {
    pub nature_code: Option<String>,
    pub cargo_type_code: Option<String>,
    pub commodity_code: Option<String>,
    pub item_classification_code: Option<String>,
}

// ─── ItemProperty ────────────────────────────────────────────────────
// XSD: ItemPropertyType
// A property or characteristic of an item

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemProperty {
    pub id: Option<String>,
    pub name: String, // 1..1 required
    pub name_code: Option<String>,
    pub test_method: Option<String>,
    pub value: Option<String>,
    pub value_quantity: Option<f64>,
    pub value_qualifier: Vec<String>,
    pub importance_code: Option<String>,
    pub list_value: Vec<String>,
    // CAC: UsabilityPeriod, ItemPropertyGroup, RangeDimension,
    // ItemPropertyRange, StandardPropertyIdentification, SubItemProperty
}

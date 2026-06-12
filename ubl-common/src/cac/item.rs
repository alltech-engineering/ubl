// UBL Item aggregates — the item of sale/trade, its identification, and properties.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::tax::TaxCategory;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub description: Option<Description>,
    pub pack_quantity: Option<PackQuantity>,
    pub pack_size_numeric: Option<Numeric>,
    pub catalogue_indicator: Option<CatalogueIndicator>,
    pub name: Option<Name>,
    pub hazardous_risk_indicator: Option<HazardousRiskIndicator>,
    pub additional_information: Option<AdditionalInformation>,
    pub item_type_code: Option<ItemTypeCode>,
    pub warranty_information: Option<WarrantyInformation>,
    pub lifecycle_stage_code: Option<LifecycleStageCode>,
    pub lifecycle_stage_description: Option<LifecycleStageDescription>,
    #[serde(default)]
    pub keyword: Vec<Keyword>,
    #[serde(default)]
    pub brand_name: Vec<BrandName>,
    #[serde(default)]
    pub model_name: Vec<ModelName>,
    pub buyers_item_identification: Option<ItemIdentification>,
    pub sellers_item_identification: Option<ItemIdentification>,
    pub manufacturers_item_identification: Option<ItemIdentification>,
    pub standard_item_identification: Option<ItemIdentification>,
    pub catalogue_item_identification: Option<ItemIdentification>,
    #[serde(default)]
    pub additional_item_identification: Vec<ItemIdentification>,
    #[serde(default)]
    pub commodity_classification: Vec<CommodityClassification>,
    #[serde(default)]
    pub item_instance: Vec<ItemInstance>,
    #[serde(default)]
    pub item_property: Vec<ItemProperty>,
    #[serde(default)]
    pub classified_tax_category: Vec<TaxCategory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemIdentification {
    pub id: ID,
    pub extended_id: Option<ID>,
    pub barcode_symbology_id: Option<BarcodeSymbologyID>,
    pub issuer_scope_id: Option<IssuerScopeID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemInstance {
    pub product_trace_id: Option<ProductTraceID>,
    pub manufacture_date: Option<ManufactureDate>,
    pub manufacture_time: Option<Time>,
    pub best_before_date: Option<BestBeforeDate>,
    pub registration_id: Option<RegistrationID>,
    pub serial_id: Option<SerialID>,
    #[serde(default)]
    pub additional_item_property: Vec<ItemProperty>,
    pub lot_identification: Option<LotIdentification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotIdentification {
    pub lot_number_id: Option<LotNumberID>,
    pub expiry_date: Option<ExpiryDate>,
    #[serde(default)]
    pub additional_item_property: Vec<ItemProperty>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemProperty {
    pub id: Option<ID>,
    pub name: Text,
    pub name_code: Option<Code>,
    pub value: Text,
    pub value_quantity: Option<Quantity>,
    #[serde(default)]
    pub value_qualifier: Vec<Text>,
    pub importance_code: Option<ImportanceCode>,
    #[serde(default)]
    pub list_value: Vec<Text>,
    pub usability_period: Option<Period>,
    #[serde(default)]
    pub item_property_group: Vec<ItemPropertyGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyGroup {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub importance_code: Option<ImportanceCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommodityClassification {
    pub item_classification_code: Option<ItemClassificationCode>,
    pub commodity_code: Option<CommodityCode>,
}

use crate::cac::period::Period;

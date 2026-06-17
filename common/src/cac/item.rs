// UBL Item aggregates — the item of sale/trade, its identification, and properties.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

use crate::cac::tax::TaxCategory;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub description: Option<Description>,
    #[serde(default)]
    pub pack_quantity: Option<PackQuantity>,
    #[serde(default)]
    pub pack_size_numeric: Option<Numeric>,
    #[serde(default)]
    pub catalogue_indicator: Option<CatalogueIndicator>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub hazardous_risk_indicator: Option<HazardousRiskIndicator>,
    #[serde(default)]
    pub additional_information: Option<AdditionalInformation>,
    #[serde(default)]
    pub item_type_code: Option<ItemTypeCode>,
    #[serde(default)]
    pub warranty_information: Option<WarrantyInformation>,
    #[serde(default)]
    pub lifecycle_stage_code: Option<LifecycleStageCode>,
    #[serde(default)]
    pub lifecycle_stage_description: Option<LifecycleStageDescription>,
    #[serde(default)]
    pub keyword: Vec<Keyword>,
    #[serde(default)]
    pub brand_name: Vec<BrandName>,
    #[serde(default)]
    pub model_name: Vec<ModelName>,
    #[serde(default)]
    pub buyers_item_identification: Option<ItemIdentification>,
    #[serde(default)]
    pub sellers_item_identification: Option<ItemIdentification>,
    #[serde(default)]
    pub manufacturers_item_identification: Option<ItemIdentification>,
    #[serde(default)]
    pub standard_item_identification: Option<ItemIdentification>,
    #[serde(default)]
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
    #[serde(default)]
    pub extended_id: Option<ID>,
    #[serde(default)]
    pub barcode_symbology_id: Option<BarcodeSymbologyID>,
    #[serde(default)]
    pub issuer_scope_id: Option<IssuerScopeID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemInstance {
    #[serde(default)]
    pub product_trace_id: Option<ProductTraceID>,
    #[serde(default)]
    pub manufacture_date: Option<ManufactureDate>,
    #[serde(default)]
    pub manufacture_time: Option<Time>,
    #[serde(default)]
    pub best_before_date: Option<BestBeforeDate>,
    #[serde(default)]
    pub registration_id: Option<RegistrationID>,
    #[serde(default)]
    pub serial_id: Option<SerialID>,
    #[serde(default)]
    pub additional_item_property: Vec<ItemProperty>,
    #[serde(default)]
    pub lot_identification: Option<LotIdentification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotIdentification {
    #[serde(default)]
    pub lot_number_id: Option<LotNumberID>,
    #[serde(default)]
    pub expiry_date: Option<ExpiryDate>,
    #[serde(default)]
    pub additional_item_property: Vec<ItemProperty>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemProperty {
    #[serde(default)]
    pub id: Option<ID>,
    pub name: Text,
    #[serde(default)]
    pub name_code: Option<Code>,
    pub value: Text,
    #[serde(default)]
    pub value_quantity: Option<Quantity>,
    #[serde(default)]
    pub value_qualifier: Vec<Text>,
    #[serde(default)]
    pub importance_code: Option<ImportanceCode>,
    #[serde(default)]
    pub list_value: Vec<Text>,
    #[serde(default)]
    pub usability_period: Option<Period>,
    #[serde(default)]
    pub item_property_group: Vec<ItemPropertyGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPropertyGroup {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub importance_code: Option<ImportanceCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommodityClassification {
    #[serde(default)]
    pub item_classification_code: Option<ItemClassificationCode>,
    #[serde(default)]
    pub commodity_code: Option<CommodityCode>,
}

use crate::cac::period::Period;

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_item() -> Item {
        Item {
            description: None,
            pack_quantity: None,
            pack_size_numeric: None,
            catalogue_indicator: None,
            name: None,
            hazardous_risk_indicator: None,
            additional_information: None,
            keyword: vec![],
            brand_name: vec![],
            model_name: vec![],
            buyers_item_identification: None,
            sellers_item_identification: None,
            manufacturers_item_identification: None,
            standard_item_identification: None,
            catalogue_item_identification: None,
            additional_item_identification: vec![],
            commodity_classification: vec![],
            item_instance: vec![],
            item_property: vec![],
            classified_tax_category: vec![],
            item_type_code: None,
            warranty_information: None,
            lifecycle_stage_code: None,
            lifecycle_stage_description: None,
        }
    }

    #[test]
    fn test_item_roundtrip() {
        let mut item = empty_item();
        item.description = Some(Description::new("Widget, Model X"));
        item.name = Some(Name::new("Widget"));
        let json = serde_json::to_string(&item).unwrap();
        let item2: Item = serde_json::from_str(&json).unwrap();
        assert_eq!(
            item.description.unwrap().value(),
            item2.description.unwrap().value()
        );
    }
}

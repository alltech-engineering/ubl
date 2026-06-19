#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::Code>,
    #[serde(default, rename = "LifeCycleStatusCode")]
    pub life_cycle_status_code: Option<cct::Code>,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "OrderableIndicator")]
    pub orderable_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: Option<cct::Text>,
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: Option<cct::Code>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<crate::Period>,
    #[serde(default, rename = "ItemComparison")]
    pub item_comparison: Vec<crate::ItemComparison>,
    #[serde(default, rename = "ComponentRelatedItem")]
    pub component_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "AccessoryRelatedItem")]
    pub accessory_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "RequiredRelatedItem")]
    pub required_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "ComplementaryRelatedItem")]
    pub complementary_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "ReplacedRelatedItem")]
    pub replaced_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
    #[serde(default, rename = "KeywordItemProperty")]
    pub keyword_item_property: Vec<crate::ItemProperty>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<crate::LineReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
}

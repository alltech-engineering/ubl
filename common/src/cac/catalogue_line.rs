#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LifeCycleStatusCode")]
    pub life_cycle_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "OrderableIndicator")]
    pub orderable_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: Option<super::cct::TextType>,
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<super::cct::TextType>,
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<Party>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<Period>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<Period>,
    #[serde(default, rename = "ItemComparison")]
    pub item_comparison: Vec<ItemComparison>,
    #[serde(default, rename = "ComponentRelatedItem")]
    pub component_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "AccessoryRelatedItem")]
    pub accessory_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "RequiredRelatedItem")]
    pub required_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "ComplementaryRelatedItem")]
    pub complementary_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "ReplacedRelatedItem")]
    pub replaced_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<ItemLocationQuantity>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "KeywordItemProperty")]
    pub keyword_item_property: Vec<ItemProperty>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<LineReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<DocumentReference>,
}

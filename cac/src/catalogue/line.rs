#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Catalogue describing a purchasable item.
///
/// UBL Dictionary Entry Name: `Catalogue Line. Details`
///
/// Generated from XSD type `CatalogueLineType`.
pub struct CatalogueLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the line in the catalogue.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A code signifying the action required to synchronize this catalogue line. Recommend codes (delete,
/// update, add)
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::Code>,
/// A code signifying the life cycle status of this catalogue line. Examples are pre-order, end of
/// production
    #[serde(default, rename = "LifeCycleStatusCode")]
    pub life_cycle_status_code: Option<cct::Code>,
/// A subdivision of a contract or tender covering this catalogue line.
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: Option<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An indicator that this catalogue line describes an orderable item (true) or is included for
/// reference purposes only (false).
    #[serde(default, rename = "OrderableIndicator")]
    pub orderable_indicator: Option<udt::Indicator>,
/// A textual description of the units in which the item described in this catalogue line can be
/// ordered.
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: Option<cct::Text>,
/// The numeric quantity of the ordering unit (and units of measure) of the catalogue line.
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: Option<cct::Quantity>,
/// The number of items that can set the order quantity increments.
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: Option<cct::Numeric>,
/// The minimum amount of the item described in this catalogue line that can be ordered.
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: Option<cct::Quantity>,
/// The maximum amount of the item described in this catalogue line that can be ordered.
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: Option<cct::Quantity>,
/// Text about a warranty (provided by WarrantyParty) for the good or service described in this
/// catalogue line.
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
/// A mutually agreed code signifying the level of packaging associated with the item described in this
/// catalogue line.
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: Option<cct::Code>,
/// The customer responsible for the contract with which this catalogue line is associated.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
/// The seller/supplier responsible for the contract with which this catalogue line is associated.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
/// The Party who is responsible for any warranty associated with the item described in this Catalogue
/// Line.
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
/// The period for which a warranty associated with the item in this catalogue line is valid.
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
/// The period for which the information in this catalogue line is valid.
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: Option<crate::Period>,
/// A combination of price and quantity used to provide price comparisons based on different sizes of
/// order.
    #[serde(default, rename = "ItemComparison")]
    pub item_comparison: Vec<crate::ItemComparison>,
/// An item that may be a component of the item in this catalogue line.
    #[serde(default, rename = "ComponentRelatedItem")]
    pub component_related_item: Vec<crate::RelatedItem>,
/// An item that may be an optional accessory of the item in this catalogue line.
    #[serde(default, rename = "AccessoryRelatedItem")]
    pub accessory_related_item: Vec<crate::RelatedItem>,
/// An item that may be required for the item in this catalogue line.
    #[serde(default, rename = "RequiredRelatedItem")]
    pub required_related_item: Vec<crate::RelatedItem>,
/// An item that may be a replacement for the item in this catalogue line.
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<crate::RelatedItem>,
/// An item that may complement the item in this catalogue line.
    #[serde(default, rename = "ComplementaryRelatedItem")]
    pub complementary_related_item: Vec<crate::RelatedItem>,
/// An item in an existing catalogue that is being replaced by the item in this catalogue line.
    #[serde(default, rename = "ReplacedRelatedItem")]
    pub replaced_related_item: Vec<crate::RelatedItem>,
/// Properties of the item in this catalogue line that are dependent on location and quantity.
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
/// A reference to a document associated with this catalogue line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// A specification of the item itself.
    #[serde(rename = "Item")]
    pub item: crate::Item,
/// A property of the item in this catalogue line.
    #[serde(default, rename = "KeywordItemProperty")]
    pub keyword_item_property: Vec<crate::ItemProperty>,
/// Reference to a Line on a Call For Tenders document.
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<crate::LineReference>,
/// One or more references to Call For Tenders documents.
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
}

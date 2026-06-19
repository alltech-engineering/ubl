#[derive(Debug, Deserialize, Serialize)]
pub struct TenderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
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
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Item")]
    pub item: Option<crate::Item>,
    #[serde(default, rename = "OfferedItemLocationQuantity")]
    pub offered_item_location_quantity: Vec<crate::ItemLocationQuantity>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<crate::RelatedItem>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
    #[serde(default, rename = "SubTenderLine")]
    pub sub_tender_line: Vec<TenderLine>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<crate::LineReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
}

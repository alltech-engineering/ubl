#[derive(Debug, Deserialize, Serialize)]
pub struct TenderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<super::cct::AmountType>,
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
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
    #[serde(default, rename = "OfferedItemLocationQuantity")]
    pub offered_item_location_quantity: Vec<ItemLocationQuantity>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<RelatedItem>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<Party>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<Period>,
    #[serde(default, rename = "SubTenderLine")]
    pub sub_tender_line: Vec<TenderLine>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<LineReference>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<DocumentReference>,
}

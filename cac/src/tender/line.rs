#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Tender.
///
/// UBL Dictionary Entry Name: `Tender Line. Details`
///
/// Generated from XSD type `TenderLineType`.
pub struct TenderLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this tender line.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of the item quoted in this tender line.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The total amount for this tender line, including allowance charges but net of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this tender line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The total tax amount for this tender line.
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
/// Text describing a unit in which the item described in this tender line can be ordered.
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: Option<cct::Text>,
/// The unit of measure and quantity of the orderable unit.
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: Option<cct::Quantity>,
/// The number of items that can set the order quantity increments.
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: Option<cct::Numeric>,
/// The minimum number of items described in this tender line that can be ordered.
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: Option<cct::Quantity>,
/// The maximum number of items described in this tender line that can be ordered.
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: Option<cct::Quantity>,
/// Text about a warranty (provided by WarrantyParty) for the good or service described in this tender
/// line.
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
/// A mutually agreed code signifying the level of packaging associated with the item described in this
/// tender line.
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: Option<cct::Code>,
/// A reference to a document associated with this tender line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// The item associated with this tender line.
    #[serde(default, rename = "Item")]
    pub item: Option<crate::Item>,
/// A set of location-specific properties (e.g., price, quantity, lead time) associated with the item
/// described in this tender line.
    #[serde(default, rename = "OfferedItemLocationQuantity")]
    pub offered_item_location_quantity: Vec<crate::ItemLocationQuantity>,
/// A catalogue item that may be a replacement for the item described in this tender line.
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: Vec<crate::RelatedItem>,
/// The Party who is responsible for any warranty described with this Tender Line.
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
/// The period for which a warranty associated with the item described in this tender line is valid.
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
/// An association to a Sub Tender Line
    #[serde(default, rename = "SubTenderLine")]
    pub sub_tender_line: Vec<TenderLine>,
/// Reference to a Line on a Call For Tenders document.
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: Option<crate::LineReference>,
/// One or more references to Call For Tenders documents.
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: Vec<crate::DocumentReference>,
}

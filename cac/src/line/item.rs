#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a line item.
///
/// UBL Dictionary Entry Name: `Line Item. Details`
///
/// Generated from XSD type `LineItemType`.
pub struct LineItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this line item, assigned by the buyer.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for this line item, assigned by the seller.
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
/// A universally unique identifier for this line item.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the status of this line item with respect to its original state.
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
/// The quantity of items associated with this line item.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The total amount for this line item, including allowance charges but net of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this line item, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The total tax amount for this line item.
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
/// The minimum quantity of the item associated with this line.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The maximum quantity of the item associated with this line.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// The minimum back order quantity of the item associated with this line (where back order is allowed).
    #[serde(default, rename = "MinimumBackorderQuantity")]
    pub minimum_backorder_quantity: Option<cct::Quantity>,
/// The maximum back order quantity of the item associated with this line (where back order is allowed).
    #[serde(default, rename = "MaximumBackorderQuantity")]
    pub maximum_backorder_quantity: Option<cct::Quantity>,
/// A code signifying the inspection requirements for the item associated with this line item.
    #[serde(default, rename = "InspectionMethodCode")]
    pub inspection_method_code: Option<cct::Code>,
/// An indicator that a partial delivery is allowed (true) or not (false).
    #[serde(default, rename = "PartialDeliveryIndicator")]
    pub partial_delivery_indicator: Option<udt::Indicator>,
/// An indicator that back order is allowed (true) or not (false).
    #[serde(default, rename = "BackOrderAllowedIndicator")]
    pub back_order_allowed_indicator: Option<udt::Indicator>,
/// The buyer's accounting cost centre for this line item, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost centre for this line item, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// Text describing a warranty (provided by WarrantyParty) for the good or service described in this
/// line item.
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
/// A delivery associated with this line item.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
/// Terms and conditions of the delivery associated with this line item.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<crate::DeliveryTerms>,
/// The Party who originates the Order to which this Line Item is related.
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<crate::Party>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<crate::Party>,
/// An ordered shipment associated with this line item.
    #[serde(default, rename = "OrderedShipment")]
    pub ordered_shipment: Vec<crate::OrderedShipment>,
/// A reference to pricing and item location information associated with this line item.
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<crate::PricingReference>,
/// An allowance or charge associated with this line item.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// The price of the item of trade associated with this line item.
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// The item of trade associated with this line item.
    #[serde(rename = "Item")]
    pub item: crate::Item,
/// The subsidiary line items that constitute the main line item, such as in a bill of materials.
    #[serde(default, rename = "SubLineItem")]
    pub sub_line_item: Vec<LineItem>,
/// The period during which the warranty associated with this line item is valid.
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
/// The Party who is responsible for any warranty associated with this Line Item.
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
/// A total amount of taxes of a particular kind applicable to this item.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// The price extension, calculated by multiplying the price per unit by the quantity of items.
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<crate::PriceExtension>,
/// A reference to a line in a document associated with this line item.
    #[serde(default, rename = "LineReference")]
    pub line_reference: Vec<LineReference>,
}

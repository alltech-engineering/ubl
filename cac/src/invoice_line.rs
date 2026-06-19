#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an Invoice.
///
/// UBL Dictionary Entry Name: `Invoice Line. Details`
///
/// Generated from XSD type `InvoiceLineType`.
pub struct InvoiceLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this invoice line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this invoice line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity (of items) on this invoice line.
    #[serde(default, rename = "InvoicedQuantity")]
    pub invoiced_quantity: Option<cct::Quantity>,
/// The total amount for this invoice line, including allowance charges but net of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this invoice line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The date of this invoice line, used to indicate the point at which tax becomes applicable.
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
/// The buyer's accounting cost centre for this invoice line, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost centre for this invoice line, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// An indicator that this invoice line is free of charge (true) or not (false). The default is false.
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: Option<udt::Indicator>,
/// An invoice period to which this invoice line applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
/// A reference to an order line associated with this invoice line.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
/// A reference to a despatch line associated with this invoice line.
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<LineReference>,
/// A reference to a receipt line associated with this invoice line.
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Vec<LineReference>,
/// A reference to a work report line associated with this invoice line.
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: Vec<LineReference>,
/// A reference to a billing document associated with this invoice line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
/// A reference to a document associated with this invoice line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// A reference to pricing and item location information associated with this invoice line.
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<PricingReference>,
/// A reference to an object, such as a subscription number, telephone number, meter, vehicle, person,
/// etc., to which this Invoice Line relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<PurchaseReference>,
/// The Party who originates the Order to which the Invoice is related.
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<Party>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
/// The Party on whose behalf this item or amount is collected.
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: Option<Party>,
/// A delivery associated with this invoice line.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<Delivery>,
/// A specification of payment terms associated with this invoice line.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
/// An allowance or charge associated with this invoice line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this invoice line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
/// A reference to a TaxTotal class describing the amount that has been withhold by the authorities,
/// e.g. if the creditor is in dept because of non paid taxes.
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<TaxTotal>,
/// The item associated with this invoice line.
    #[serde(rename = "Item")]
    pub item: Item,
/// The price of the item associated with this invoice line.
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
/// Terms and conditions of the delivery associated with this invoice line.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<DeliveryTerms>,
/// An invoice line subsidiary to this invoice line.
    #[serde(default, rename = "SubInvoiceLine")]
    pub sub_invoice_line: Vec<InvoiceLine>,
/// The price extension, calculated by multiplying the price per unit by the quantity of items on this
/// invoice line.
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<PriceExtension>,
}

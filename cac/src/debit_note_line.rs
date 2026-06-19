#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Debit Note.
///
/// UBL Dictionary Entry Name: `Debit Note Line. Details`
///
/// Generated from XSD type `DebitNoteLineType`.
pub struct DebitNoteLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this debit note line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this debit note line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of Items debited in this debit note line.
    #[serde(default, rename = "DebitedQuantity")]
    pub debited_quantity: Option<cct::Quantity>,
/// The total amount for this debit note line, including allowance charges but net of taxes.
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: cct::Amount,
/// The total amount for this debit note line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The date of this debit note line, used to indicate the point at which tax becomes applicable.
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
/// The buyer's accounting cost centre for this debit note line, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost centre for this debit note line, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// An indicator that this Debit Note Line is free of charge (true) or not (false). The default is
/// false.
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: Option<udt::Indicator>,
/// An invoice period to which this Debit Note Line applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
/// A reference to an Order Line associated with this Debit Note Line.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
/// A reason for the debit.
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<Response>,
/// A reference to a despatch line associated with this debit note line.
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<LineReference>,
/// A reference to a receipt line associated with this debit note line.
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Vec<LineReference>,
/// A reference to a work report line associated with this debit note line.
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: Vec<LineReference>,
/// A reference to a billing document associated with this debit note line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
/// A reference to a document associated with this debit note line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// A reference to pricing and item location information associated with this debit note line.
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<PricingReference>,
/// The Party who originated the Order to which the Debit Note is related.
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<Party>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
/// The Party on whose behalf this item or amount is collected.
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: Option<Party>,
/// A delivery associated with this debit note line.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<Delivery>,
/// A specification of payment terms associated with this Debit Note Line.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
/// A total amount of taxes of a particular kind applicable to this debit note line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
/// A reference to a TaxTotal class describing the amount that has been withhold by the authorities,
/// e.g. if the creditor is in dept because of non paid taxes.
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<TaxTotal>,
/// An allowance or charge associated with this debit note.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
/// The item associated with this debit note line.
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
/// The price of the item associated with this debit note line.
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
/// Terms and conditions of a delivery associated with this Credit Note Line.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<DeliveryTerms>,
/// A recursive description of a debit note line subsidiary to this debit note line.
    #[serde(default, rename = "SubDebitNoteLine")]
    pub sub_debit_note_line: Vec<DebitNoteLine>,
/// The price extension, calculated by multiplying the price per unit by the quantity of items on this
/// Debit Note Line.
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<PriceExtension>,
}

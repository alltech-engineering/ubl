use serde::{Deserialize, Serialize};


include!("account.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Credit Note or Self Billed Credit Note.
///
/// UBL Dictionary Entry Name: `Credit Note Line. Details`
///
/// Generated from XSD type `CreditNoteLineType`.
pub struct CreditNoteLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this credit note line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for this credit note line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of items credited in this credit note line.
    #[serde(default, rename = "CreditedQuantity")]
    pub credited_quantity: Option<cct::Quantity>,
/// The total amount for this credit note line, including allowance charges but exclusive of taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this credit note line, including all allowances, charges and taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The date of this credit note line, used to indicate the point at which tax becomes applicable.
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
/// The buyer's accounting cost centre for this credit note line, expressed as a code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The buyer's accounting cost centre for this credit note line, expressed as text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// An indicator that this credit note line is free of charge (true) or not (false). The default is
/// false.
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: Option<udt::Indicator>,
/// An invoice period to which this credit note line applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<crate::Period>,
/// A reference to an order line associated with this credit note line.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<crate::OrderLineReference>,
/// A reason for the credit.
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<crate::Response>,
/// A reference to a despatch line associated with this credit note line.
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<crate::LineReference>,
/// A reference to a receipt line associated with this credit note line.
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Vec<crate::LineReference>,
/// A reference to a work report line associated with this credit note line.
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: Vec<crate::LineReference>,
/// A reference to a billing document associated with this credit note line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<crate::BillingReference>,
/// A reference to a document associated with this credit note line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// A reference to pricing and item location information associated with this credit note line.
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<crate::PricingReference>,
/// A reference to an object, such as a subscription number, telephone number, meter, vehicle, person,
/// etc., to which this Credit Note Line relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<crate::PurchaseReference>,
/// The Party who originates the Order to which the Credit Note is related.
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<crate::Party>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<crate::Party>,
/// The Party on whose behalf this item or amount is collected.
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: Option<crate::Party>,
/// A delivery associated with this credit note line.
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
/// A specification of payment terms associated with this credit note line.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
/// A total amount of taxes of a particular kind applicable to this credit note line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// A reference to a TaxTotal class describing the amount that has been withhold by the authorities,
/// e.g. if the creditor is in dept because of non paid taxes.
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<crate::TaxTotal>,
/// An allowance or charge associated with this credit note.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// The item associated with this credit note line.
    #[serde(default, rename = "Item")]
    pub item: Option<crate::Item>,
/// The price of the item associated with this credit note line.
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// Terms and conditions of a delivery associated with this credit note line.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<crate::DeliveryTerms>,
/// A class defining one or more Credit Note Lines detailing the credit note line.
    #[serde(default, rename = "SubCreditNoteLine")]
    pub sub_credit_note_line: Vec<CreditNoteLine>,
/// The price extension, calculated by multiplying the price per unit by the quantity of items on this
/// credit note line.
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<crate::PriceExtension>,
}

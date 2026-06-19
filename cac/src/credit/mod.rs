use serde::{Deserialize, Serialize};


include!("account.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditNoteLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "CreditedQuantity")]
    pub credited_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<crate::Period>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<crate::OrderLineReference>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<crate::Response>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<crate::LineReference>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Vec<crate::LineReference>,
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: Vec<crate::LineReference>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<crate::BillingReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<crate::PricingReference>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<crate::PurchaseReference>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<crate::Party>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<crate::Party>,
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: Option<crate::Party>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "Item")]
    pub item: Option<crate::Item>,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<crate::DeliveryTerms>,
    #[serde(default, rename = "SubCreditNoteLine")]
    pub sub_credit_note_line: Vec<CreditNoteLine>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<crate::PriceExtension>,
}

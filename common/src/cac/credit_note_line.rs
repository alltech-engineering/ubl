#[derive(Debug, Deserialize, Serialize)]
pub struct CreditNoteLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "CreditedQuantity")]
    pub credited_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<super::cct::TextType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<Response>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Vec<LineReference>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Vec<LineReference>,
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: Vec<LineReference>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<PricingReference>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<PurchaseReference>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<Party>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: Option<Party>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<Delivery>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default, rename = "SubCreditNoteLine")]
    pub sub_credit_note_line: Vec<CreditNoteLine>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<PriceExtension>,
}

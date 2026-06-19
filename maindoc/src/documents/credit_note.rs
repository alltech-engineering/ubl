#[derive(Debug, Deserialize, Serialize)]
pub struct CreditNote {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "DueDate")]
    pub due_date: Option<udt::DateTime>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTime>,
    #[serde(default, rename = "CreditNoteTypeCode")]
    pub credit_note_type_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::Code>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::Code>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: Option<cct::Code>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: Option<cct::Text>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: Option<cct::Code>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: Vec<cac::Response>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Vec<cac::ProjectReference>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: Vec<cac::BuyerAssignedReference>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Vec<cac::PurchaseReference>,
    #[serde(default, rename = "Annotation")]
    pub annotation: Vec<cac::Annotation>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: Option<cac::Party>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<cac::DeliveryTerms>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<cac::TaxTotal>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
    #[serde(default, rename = "CollectionCreditNoteLine")]
    pub collection_credit_note_line: Vec<cac::CreditNoteLine>,
    #[serde(default, rename = "CreditNoteLine")]
    pub credit_note_line: Vec<cac::CreditNoteLine>,
}

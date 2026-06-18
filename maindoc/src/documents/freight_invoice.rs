#[derive(Debug, Deserialize, Serialize)]
pub struct FreightInvoice {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "InvoiceTypeCode")]
    pub invoice_type_code: Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::NumericType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<cac::Shipment>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Vec<cac::ProjectReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerParty,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: Option<cac::Party>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: Vec<cac::Payment>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: Vec<cac::TaxTotal>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: Vec<cac::InvoiceLine>,
}

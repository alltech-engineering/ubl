#[derive(Debug, Deserialize, Serialize)]
pub struct OrderChange {
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
    #[serde(default, rename = "ID")]
    pub id: Option<cct::IdentifierType>,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(rename = "SequenceNumberID")]
    pub sequence_number_id: cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "RequestedInvoiceCurrencyCode")]
    pub requested_invoice_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(rename = "OrderReference")]
    pub order_reference: cac::OrderReference,
    #[serde(default, rename = "QuotationDocumentReference")]
    pub quotation_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<cac::DeliveryTerms>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Option<cac::TransactionConditions>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: Option<cac::Country>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "AnticipatedMonetaryTotal")]
    pub anticipated_monetary_total: Option<cac::MonetaryTotal>,
    #[serde(default, rename = "OrderLine")]
    pub order_line: Vec<cac::OrderLine>,
}

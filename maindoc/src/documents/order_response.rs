#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponse {
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
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "OrderResponseCode")]
    pub order_response_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::Code>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: Option<cct::Code>,
    #[serde(default, rename = "TotalPackagesQuantity")]
    pub total_packages_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::Text>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Vec<cac::OrderReference>,
    #[serde(default, rename = "OrderDocumentReference")]
    pub order_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "OrderChangeDocumentReference")]
    pub order_change_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<cac::DeliveryTerms>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Option<cac::TransactionConditions>,
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
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: Option<cac::MonetaryTotal>,
    #[serde(default, rename = "OrderLine")]
    pub order_line: Vec<cac::OrderLine>,
}

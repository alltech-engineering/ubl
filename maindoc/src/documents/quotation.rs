#[derive(Debug, Deserialize, Serialize)]
pub struct Quotation {
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
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: Option<cct::CodeType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "RequestForQuotationDocumentReference")]
    pub request_for_quotation_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<cac::Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<cac::DeliveryTerms>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Option<cac::PaymentMeans>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Option<cac::TransactionConditions>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: Option<cac::Country>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(rename = "QuotedMonetaryTotal")]
    pub quoted_monetary_total: cac::MonetaryTotal,
    #[serde(default, rename = "QuotationLine")]
    pub quotation_line: Vec<cac::QuotationLine>,
}

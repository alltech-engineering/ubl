#[derive(Debug, Deserialize, Serialize)]
pub struct Statement {
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
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::Code,
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalBalanceAmount")]
    pub total_balance_amount: Option<cct::Amount>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "StatementTypeCode")]
    pub statement_type_code: Option<cct::Code>,
    #[serde(default, rename = "StatementPeriod")]
    pub statement_period: Option<cac::Period>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerParty,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "StatementLine")]
    pub statement_line: Vec<cac::StatementLine>,
}

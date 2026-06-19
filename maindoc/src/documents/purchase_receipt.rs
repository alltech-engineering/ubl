#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReceipt {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: Option<udt::DateTime>,
    #[serde(default, rename = "TransactionTime")]
    pub transaction_time: Option<udt::DateTime>,
    #[serde(default, rename = "PurchaseDate")]
    pub purchase_date: Option<udt::DateTime>,
    #[serde(default, rename = "PurchaseTime")]
    pub purchase_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Vec<cac::PurchaseReference>,
    #[serde(default, rename = "SalesDocumentReference")]
    pub sales_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "CashierContact")]
    pub cashier_contact: Option<cac::Contact>,
    #[serde(default, rename = "CashRegister")]
    pub cash_register: Option<cac::CashRegister>,
    #[serde(default, rename = "PointOfSaleLocation")]
    pub point_of_sale_location: Option<cac::Location>,
    #[serde(default, rename = "PointOfSaleContact")]
    pub point_of_sale_contact: Option<cac::Contact>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<cac::Delivery>,
    #[serde(default, rename = "Payment")]
    pub payment: Vec<cac::Payment>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
    #[serde(default, rename = "PurchaseReceiptLine")]
    pub purchase_receipt_line: Vec<cac::PurchaseReceiptLine>,
}

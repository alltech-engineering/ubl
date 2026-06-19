#[derive(Debug, Deserialize, Serialize)]
/// A receipt for a purchase made with cash or cash equivalents.
///
/// UBL Dictionary Entry Name: `Purchase Receipt. Details`
///
/// Generated from XSD type `PurchaseReceiptType`.
pub struct PurchaseReceipt {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this purchase receipt, assigned by the seller.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date when the purchase receipt was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time of day when the purchase receipt was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The date when the purchase transaction was initiated.
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: Option<udt::DateTime>,
/// The time of day when the purchase transaction was initiated.
    #[serde(default, rename = "TransactionTime")]
    pub transaction_time: Option<udt::DateTime>,
/// The date when the purchase took place.
    #[serde(default, rename = "PurchaseDate")]
    pub purchase_date: Option<udt::DateTime>,
/// The time of day when the purchase took place.
    #[serde(default, rename = "PurchaseTime")]
    pub purchase_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the default currency for this document.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// A reference to an object, such as a subscription number, telephone number, meter, vehicle, person,
/// etc., to which this purchase relates.
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Vec<cac::PurchaseReference>,
/// A reference to the sales document to which this purchase receipt is related.
    #[serde(default, rename = "SalesDocumentReference")]
    pub sales_document_reference: Option<cac::DocumentReference>,
/// A reference to an additional document associated with this purchase receipt.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The accounting supplier party.
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
/// The accounting customer party.
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
/// The cashier who handled the purchase at the point of sales.
    #[serde(default, rename = "CashierContact")]
    pub cashier_contact: Option<cac::Contact>,
/// The cash register that was used for this purchase.
    #[serde(default, rename = "CashRegister")]
    pub cash_register: Option<cac::CashRegister>,
/// The location of the point of sale where this purchase took place.
    #[serde(default, rename = "PointOfSaleLocation")]
    pub point_of_sale_location: Option<cac::Location>,
/// The contact person at the point of sale where this purchase took place.
    #[serde(default, rename = "PointOfSaleContact")]
    pub point_of_sale_contact: Option<cac::Contact>,
/// The delivery associated with this purchase.
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<cac::Delivery>,
/// One or more payments for this purchase.
    #[serde(default, rename = "Payment")]
    pub payment: Vec<cac::Payment>,
/// One or more payment means used to pay for this purchase, with their associated payments.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<cac::PaymentMeans>,
/// A discount or charge that applies to a price component.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
/// The exchange rate between the document currency and the tax currency.
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: Option<cac::ExchangeRate>,
/// The exchange rate between the document currency and the pricing currency.
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: Option<cac::ExchangeRate>,
/// The total amount of a specific type of tax.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// The total amount payable on the Invoice, including Allowances, Charges, and Taxes.
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotal,
/// One or more line items that describe this purchase.
    #[serde(default, rename = "PurchaseReceiptLine")]
    pub purchase_receipt_line: Vec<cac::PurchaseReceiptLine>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A report on the quantities of each item that are, or will be, in stock. This document is sent by a
/// Buyer (for example a retailer) to a Seller (for example a producer).
///
/// UBL Dictionary Entry Name: `Inventory Report. Details`
///
/// Generated from XSD type `InventoryReportType`.
pub struct InventoryReport {
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
/// An identifier for the Inventory Report, assigned by the Issuer.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time at which the Inventory Report was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the currency in which the Document is presented. This may be the same currency as
/// the pricing or as the tax.
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
/// The period covered by this report.
    #[serde(default, rename = "InventoryPeriod")]
    pub inventory_period: Option<cac::Period>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The retailer, who sends this message.
    #[serde(rename = "RetailerCustomerParty")]
    pub retailer_customer_party: cac::CustomerParty,
/// The Party that is reporting the inventory details.
    #[serde(rename = "InventoryReportingParty")]
    pub inventory_reporting_party: cac::Party,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// A line representing a particular item of sale and associated with a line in the Catalogue.
    #[serde(default, rename = "InventoryReportLine")]
    pub inventory_report_line: Vec<cac::InventoryReportLine>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A document used to specify the thresholds for forecast variance, product activity, and performance
/// history beyond which exceptions will be triggered.
///
/// UBL Dictionary Entry Name: `Exception Criteria. Details`
///
/// Generated from XSD type `ExceptionCriteriaType`.
pub struct ExceptionCriteria {
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
/// An identifier for this document, assigned by the sender.
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
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this document.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The period of time during which the Exception Criteria is valid.
    #[serde(rename = "ValidityPeriod")]
    pub validity_period: cac::Period,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// A document used to specify the thresholds for forecast variance, product activity, and performance
/// history beyond which exceptions will be triggered.
    #[serde(default, rename = "ExceptionCriteriaLine")]
    pub exception_criteria_line: Vec<cac::ExceptionCriteriaLine>,
}

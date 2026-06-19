#[derive(Debug, Deserialize, Serialize)]
/// A document used to describe the receipt of goods and services or as a reply to a despatch advice.
///
/// UBL Dictionary Entry Name: `Receipt Advice. Details`
///
/// Generated from XSD type `ReceiptAdviceType`.
pub struct ReceiptAdvice {
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
/// Identifies a user-defined profile of the subset of UBL being used.
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
/// A code signifying the status of the Receipt Advice with respect to its original state. This code may
/// be used if the document precedes the event and is subsequently found to be incorrect and in need of
/// cancellation or revision.
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// A code signifying the type of the Receipt Advice.
    #[serde(default, rename = "ReceiptAdviceTypeCode")]
    pub receipt_advice_type_code: Option<cct::Code>,
/// A code to specify the acceptance or rejection of the delivery.
    #[serde(default, rename = "DeliveryAcceptanceCode")]
    pub delivery_acceptance_code: Option<cct::Code>,
/// The reason for a rejection, expressed as a code.
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: Option<cct::Code>,
/// The reason for a rejection, expressed as text.
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: Vec<cct::Text>,
/// A code signifying the action that the delivery party wishes the despatch party to take in the case
/// of a rejection.
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The number of Receipt Lines in this document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// A reference to an Order associated with this Receipt Advice.
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Vec<cac::OrderReference>,
/// A reference to a Despatch Advice associated with this document.
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The customer party.
    #[serde(rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: cac::CustomerParty,
/// The supplier party.
    #[serde(rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: cac::SupplierParty,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// Details about the Shipment.
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
/// A line detailing a kind of item received.
    #[serde(default, rename = "ReceiptLine")]
    pub receipt_line: Vec<cac::ReceiptLine>,
}

#[derive(Debug, Deserialize, Serialize)]
/// A document used to cancel an entire fulfilment document (Despatch Advice or Receipt Advice).
///
/// UBL Dictionary Entry Name: `Fulfilment Cancellation. Details`
///
/// Generated from XSD type `FulfilmentCancellationType`.
pub struct FulfilmentCancellation {
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
/// The reason for cancellation of the referenced document.
    #[serde(default, rename = "CancellationNote")]
    pub cancellation_note: Vec<cct::Text>,
/// A reference to a Despatch Advice associated with this document.
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: Vec<cac::DocumentReference>,
/// A reference to a Receipt Advice associated with this document.
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: Vec<cac::DocumentReference>,
/// A reference to an Order document associated with the referenced Despatch or Receipt Advice(s).
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Vec<cac::OrderReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// The contracts or framework agreements with which the referenced fulfilment document is associated.
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The delivery party.
    #[serde(default, rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: Option<cac::CustomerParty>,
/// The despatch party.
    #[serde(default, rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: Option<cac::SupplierParty>,
/// The originator party
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
}

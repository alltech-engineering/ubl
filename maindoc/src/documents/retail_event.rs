#[derive(Debug, Deserialize, Serialize)]
/// A document used to specify basic information about retail events (such as promotions, product
/// introductions, and community or environmental events) that affect supply or demand.
///
/// UBL Dictionary Entry Name: `Retail Event. Details`
///
/// Generated from XSD type `RetailEventType`.
pub struct RetailEvent {
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
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A title, theme, slogan, or other identifier for the event for use by trading partners.
    #[serde(default, rename = "RetailEventName")]
    pub retail_event_name: Option<cct::Text>,
/// Describes the logical state of the discrete activity affecting supply or demand in the supply chain
    #[serde(rename = "RetailEventStatusCode")]
    pub retail_event_status_code: cct::Code,
/// An event tracking identifier assigned by the seller.
    #[serde(default, rename = "SellerEventID")]
    pub seller_event_id: Option<cct::Identifier>,
/// An event tracking identifier assigned by the buyer.
    #[serde(default, rename = "BuyerEventID")]
    pub buyer_event_id: Option<cct::Identifier>,
/// Definition of the discrete activity affecting supply or demand in the supply chain
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period during which the event takes place.
    #[serde(rename = "Period")]
    pub period: cac::Period,
/// A reference to a Forecast document associated with this event.
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Retail Event.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Retail Event.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The buyer.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// A comment regarding the event.
    #[serde(default, rename = "EventComment")]
    pub event_comment: Vec<cac::EventComment>,
/// The description of a promotional event associated with this event.
    #[serde(default, rename = "PromotionalEvent")]
    pub promotional_event: Option<cac::PromotionalEvent>,
/// A miscellaneous event associated with this event.
    #[serde(default, rename = "MiscellaneousEvent")]
    pub miscellaneous_event: Option<cac::MiscellaneousEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetailEvent {
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
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "RetailEventName")]
    pub retail_event_name: Option<cct::TextType>,
    #[serde(rename = "RetailEventStatusCode")]
    pub retail_event_status_code: cct::CodeType,
    #[serde(default, rename = "SellerEventID")]
    pub seller_event_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "BuyerEventID")]
    pub buyer_event_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(rename = "Period")]
    pub period: cac::Period,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "EventComment")]
    pub event_comment: Vec<cac::EventComment>,
    #[serde(default, rename = "PromotionalEvent")]
    pub promotional_event: Option<cac::PromotionalEvent>,
    #[serde(default, rename = "MiscellaneousEvent")]
    pub miscellaneous_event: Option<cac::MiscellaneousEvent>,
}

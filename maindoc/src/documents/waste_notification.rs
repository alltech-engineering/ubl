#[derive(Debug, Deserialize, Serialize)]
/// A document used to notify the competent authorities of planned movements of waste.
///
/// UBL Dictionary Entry Name: `Waste Notification. Details`
///
/// Generated from XSD type `WasteNotificationType`.
pub struct WasteNotification {
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
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code signifying the type of this Waste Notification.
    #[serde(default, rename = "WasteNotificationTypeCode")]
    pub waste_notification_type_code: Option<cct::Code>,
/// Estimated total number of shipments for this Notification Document.
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party sending this document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party receiving this document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The Party responsible for providing the Waste Notification information and acting in the role of
/// notifier in the waste movement process.
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::Party,
/// The competent authority granting a permit to export or import the waste.
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: Vec<cac::Party>,
/// The Party disposing of the waste material.
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: Option<cac::Party>,
/// The Party recovering the waste material.
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: Option<cac::Party>,
/// The Party producing the waste material.
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::Party,
/// The relevant shipment information describing the planned transport and the waste material.
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
/// A reference to a relevant document associated with this Waste Movement.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
}
